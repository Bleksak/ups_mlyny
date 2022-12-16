package mlyn.model;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;

import javafx.scene.paint.Color;

public class Client extends Thread {
    private Socket socket;
    private InputStream is;
    private OutputStream os;

    private Machine machine = new Machine();
    private Color color;
    private static int readLimit = 4096;

    private boolean running = true;

    ConcurrentLinkedQueue<Message> messageQueue = new ConcurrentLinkedQueue<>();
    ConcurrentLinkedQueue<Message> sendQueue = new ConcurrentLinkedQueue<>();

    public Client(String ip, short port) throws IOException {
        socket = new Socket(ip, port);

        is = socket.getInputStream();
        os = socket.getOutputStream();
        this.start();
    }

    public void disconnect() {
        try {
            running = false;
            socket.close();
        } catch (IOException e) {
        }
    }

    public Machine getMachine() {
        return machine;
    }

    public Color getColor() {
        return color;
    }

    // This doesn't need to be synchronized cause we only call it once
    public void setColor(Color color) {
        this.color = color;
    }

    public boolean reconnect() throws IOException {
        long start = System.currentTimeMillis();
        long timeout = 5000;

        while(!socket.isConnected()) {
            if(System.currentTimeMillis() - start <= timeout) {
                return false;
            }

            socket = new Socket(Config.ip, Config.port);
            is = socket.getInputStream();
            os = socket.getOutputStream();
        }

        return true;
    }

    public void run() {
        while(running) {
            if(socket.isConnected()) {
                readMessage();
            }
            
            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    private Integer readInt() throws IOException {

        byte[] bytes = is.readNBytes(Integer.BYTES);
        if(bytes.length != Integer.BYTES) {
            return null;
        }

        return ByteBuffer.wrap(bytes).getInt() - Integer.BYTES;
    }

    public void readMessage() {
        try {
            Integer sizeInteger = readInt();
            if(sizeInteger == null || sizeInteger.intValue() <= 0) {
                System.out.println("message size == 0");
                messageQueue.add(new Message(MessageType.SERVER_CRASH, null));
                return;
            }

            int size = sizeInteger.intValue();

            if(size > readLimit) {
                System.out.println(size);
                System.out.println("message is too long!");
                // message too long
                return;
            }

            byte[] message = is.readNBytes(size);
            if(message.length != size) {
                System.out.println("message length != message size");
                messageQueue.add(new Message(MessageType.SERVER_CRASH, null));
                return;
            }
            ByteBuffer buffer = ByteBuffer.wrap(message);

            int remaining = buffer.remaining();
            if(remaining < Integer.BYTES) {
                // message too short
                System.out.println("message too short");
                messageQueue.add(new Message(MessageType.SERVER_CRASH, null));
                return;
            }

            int typeInt = buffer.getInt();
            MessageType type = MessageType.valueOf(typeInt);

            if(type == MessageType.INVALID) {
                // invalid message
                System.out.println("invalid message");
                messageQueue.add(new Message(MessageType.SERVER_CRASH, null));
                return;
            }

            System.out.println(size);

            int dataLength = size - Integer.BYTES;

            byte[] data = new byte[dataLength];

            if(dataLength > 0) {
                buffer.get(data);
            }

            if(type == MessageType.PING) {
                sendQueue.add(new Message(type, data));
                return;
            }

            if(type == MessageType.PONG) {
                return;
            }

            System.out.println("message read ok");
            System.out.println("Adding: " + type.name());
            messageQueue.add(new Message(type, data));
        } catch(IOException e) {
            return;
        }
    }

    void splitMessages(List<Byte> byteList) {
        if(byteList == null) {
            messageQueue.add(new Message(MessageType.SERVER_CRASH, new byte[0]));
            return;
        }

        byte[] bytes = new byte[byteList.size()];
        for(int i = 0; i < byteList.size(); ++i) {
            bytes[i] = byteList.get(i).byteValue();
        }

    }

    public void send(Message msg) throws IOException {
        System.out.println("sending msg: " + msg.type().name());
        os.write(msg.serialize());
    }

    public Message getMessage(MessageType... types) throws InterruptedException {
        while(true) {
            if(Thread.currentThread().isInterrupted()) {
                throw new InterruptedException();
            }

            synchronized(this.messageQueue) {
                if(messageQueue.isEmpty()) {
                    continue;
                }
                if(types.length == 0) {
                    return messageQueue.remove();
                }

                Message msg = messageQueue.peek();
                for(MessageType type : types) {
                    if(msg.type() == type) {
                        return messageQueue.remove();
                    }
                }
            }
        }
    }
}
