package mlyn.model;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;

import javafx.scene.paint.Color;

public class Client extends Thread {
    private Socket socket;
    private InputStream is;
    private OutputStream os;

    private Machine machine = new Machine();
    private Color color;

    private boolean running = true;

    ConcurrentLinkedQueue<Message> messageQueue = new ConcurrentLinkedQueue<>();
    ConcurrentLinkedQueue<Message> sendQueue = new ConcurrentLinkedQueue<>();

    public Client() throws IOException {
        socket = new Socket(Config.ip, Config.port);

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
                splitMessages(readAll());
            }
        }
    }

    public List<Byte> readAll() {
        List<Byte> bytes = new ArrayList<>();

        try {
            byte[] byteArray = new byte[512];
            int read = is.read(byteArray, 0, byteArray.length);

            for(int i = 0; i < read; ++i) {
                bytes.add(byteArray[i]);
            }

            if(read < 0) {
                return null;
            }

            while(true) {
                byteArray = new byte[512];

                if(is.available() <= 0) {
                    return bytes;
                }

                read = is.read(byteArray, 0, byteArray.length);

                for(int i = 0; i < read; ++i) {
                    bytes.add(byteArray[i]);
                }

                if(read <= 0) {
                    return bytes;
                }
            }

        } catch(IOException e) {
            return null;
        }
    }

    void splitMessages(List<Byte> byteList) {
        if(byteList == null) {
            return;
        }

        byte[] bytes = new byte[byteList.size()];
        for(int i = 0; i < byteList.size(); ++i) {
            bytes[i] = byteList.get(i).byteValue();
        }

        ByteBuffer buffer = ByteBuffer.wrap(bytes);

        while(true) {
            int remaining = buffer.remaining();
            if(remaining < 2 * Integer.BYTES) {
                break;
            }

            int size = buffer.getInt() - 2 * Integer.BYTES;
            if(size < 0) {
                continue;
            }


            int typeInt = buffer.getInt();
            MessageType type = MessageType.valueOf(typeInt);

            if(type == MessageType.INVALID) {
                continue;
            }

            if(buffer.remaining() < size) {
                continue;
            }

            byte[] data = new byte[size];
            buffer.get(data);

            if(type == MessageType.PING) {
                sendQueue.add(new Message(type, data));
                continue;
            }

            if(type == MessageType.PONG) {
                // TODO: handle pong
                continue;
            }

            messageQueue.add(new Message(type, data));
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
