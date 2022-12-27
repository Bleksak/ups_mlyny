package mlyn.model;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;

import javafx.scene.paint.Color;

class Ping {
    public long start;
    public long sleep;
    public boolean pinged;
}

public class Client extends Thread {
    private String ip;
    private short port;
    private Socket socket;
    private InputStream is;
    private OutputStream os;

    private Machine machine = new Machine();
    private Color color;
    private static int readLimit = 4096;

    private boolean running = true;

    private Ping ping;

    ConcurrentLinkedQueue<Message> messageQueue = new ConcurrentLinkedQueue<>();
    ConcurrentLinkedQueue<Message> sendQueue = new ConcurrentLinkedQueue<>();

    public Client(String ip, short port) throws IOException {
        this.ip = ip;
        this.port = port;

        socket = new Socket();
        socket.connect(new InetSocketAddress(ip, port), 5000);
        socket.setSoTimeout(1000);
        ping = new Ping();

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

            socket = new Socket();
            socket.connect(new InetSocketAddress(ip, port), 5000);
            socket.setSoTimeout(1000);
            ping = new Ping();

            is = socket.getInputStream();
            os = socket.getOutputStream();

            sendMessage(new Message(MessageType.PING, ""));
        }

        return true;
    }

    public void run() {
        while(running) {
            if(socket.isConnected()) {
                synchronized(ping) {
                    if(!ping.pinged && System.currentTimeMillis() - ping.sleep >= 3000) {
                        System.out.println("sending ping");
                        sendMessage(new Message(MessageType.PING, ""));
                        ping.pinged = true;
                        ping.start = System.currentTimeMillis();
                    }
                }

                try {
                    sendMessages();
                } catch(IOException ex) {
                }
                catch(InterruptedException e) {}
                readMessage();
            }
            
            try {
                Thread.sleep(20);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    private void sendMessages() throws IOException, InterruptedException {
        synchronized(this.sendQueue) {
            while(!sendQueue.isEmpty()) {
                Message msg = sendQueue.remove();
                send(msg);
                Thread.sleep(80);
            }
        }
    }

    public void sendMessage(Message msg) {
        synchronized(sendQueue) {
            sendQueue.add(msg);
        }
    }

    private Integer readLen() throws IOException {
        StringBuilder chars = new StringBuilder();

        if(is.available() <= 1) {
            return -1;
        }

        while(true) {
            int read = is.read();

            if(read == -1) {
                return null;
            }

            if((char) read == ';') {
                break;
            }

            if((char) read == '\n') {
                return -1;
            }

            if(chars.length() >= 4) {
                return null;
            }

            chars.append((char)read);
        }

        try {
            int value = Integer.parseInt(chars.toString());
            if(value <= 0) {
                return null;
            }

            return value;
        } catch(NumberFormatException e) {
            return null;
        }

        // byte[] bytes = is.readNBytes(Integer.BYTES);
        // if(bytes.length != Integer.BYTES) {
        //     return null;
        // }

        // return ByteBuffer.wrap(bytes).getInt() - Integer.BYTES;
    }

    public void readMessage() {
        try {
            Integer sizeInteger = readLen();

            if(sizeInteger == null || sizeInteger.intValue() == 0) {
                System.out.println("message size == 0");
                messageQueue.add(new Message(MessageType.CRASH, ""));
                return;
            }

            if(sizeInteger.intValue() == -1) {
                return;
            }

            int size = sizeInteger.intValue();

            if(size > readLimit) {
                System.out.println(size);
                System.out.println("message is too long!");
                messageQueue.add(new Message(MessageType.CRASH, ""));
                return;
            }

            byte[] message = is.readNBytes(size);
            if(message.length != size) {
                System.out.println("message length != message size");
                messageQueue.add(new Message(MessageType.CRASH, ""));
                return;
            }

            String msgString = new String(message, StandardCharsets.UTF_8);
            if(msgString.charAt(msgString.length() - 1) == '\n') {
                msgString = msgString.substring(0, msgString.length() - 1);
            }

            System.out.println("GOT MESSAGE: " + msgString);

            String[] splitted = msgString.split(";");

            // ByteBuffer buffer = ByteBuffer.wrap(message);

            // int remaining = buffer.remaining();
            // if(remaining < Integer.BYTES) {
            //     // message too short
            //     System.out.println("message too short");
            //     messageQueue.add(new Message(MessageType.CRASH, (byte[][])null));
            //     return;
            // }

            String typeString = splitted[0];

            // int typeInt = buffer.getInt();
            MessageType type = MessageType.valueOfString(typeString);

            if(type == MessageType.INVALID) {
                // invalid message
                System.out.println("invalid message");
                messageQueue.add(new Message(MessageType.CRASH, ""));
                return;
            }

            System.out.println(size);

            // int dataLength = size - Integer.BYTES;

            // byte[] data = new byte[dataLength];

            // if(dataLength > 0) {
            //     buffer.get(data);
            // }

            if(type == MessageType.PING) {
                sendMessage(new Message(MessageType.PONG, ""));
                return;
            }


            if(type == MessageType.PONG) {
                synchronized(ping) {
                    ping.sleep = System.currentTimeMillis();
                    ping.pinged = false;
                }
                return;
            }

            String[] data = Arrays.copyOfRange(splitted, 1, splitted.length);

            System.out.println("message read ok");

            Message msg = new Message(type, data);

            if(!machine.validateMessage(msg)) {
                System.out.println("bad message, crashing");
                messageQueue.add(new Message(MessageType.CRASH, ""));
                return;
            }

            System.out.println("Adding: " + type.name());
            messageQueue.add(new Message(type, data));
        } catch(IOException e) {
            return;
        }
    }

    void splitMessages(List<Byte> byteList) {
        if(byteList == null) {
            messageQueue.add(new Message(MessageType.CRASH, ""));
            return;
        }

        byte[] bytes = new byte[byteList.size()];
        for(int i = 0; i < byteList.size(); ++i) {
            bytes[i] = byteList.get(i).byteValue();
        }
    }

    private void send(Message msg) throws IOException {
        byte[] serialized = msg.serialize();
        System.out.println("sending msg: " + new String(serialized, StandardCharsets.UTF_8));
        os.write(serialized);
        os.flush();
    }

    public Message getMessage(MessageType... types) throws InterruptedException {
        while(true) {
            if(Thread.currentThread().isInterrupted()) {
                throw new InterruptedException();
            }

            synchronized(ping) {
                if(ping.pinged && System.currentTimeMillis() - ping.start >= 5000) {
                    return new Message(MessageType.CRASH, "");
                }
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

            Thread.sleep(100);
        }
    }
}
