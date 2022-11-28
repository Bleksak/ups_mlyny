package mlyn.model;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;

public class Client {
    private final Socket socket;
    private final InputStream is;
    private final OutputStream os;
    private Machine machine;

    ConcurrentLinkedQueue<Message> messageQueue = new ConcurrentLinkedQueue<>();

    public Client() throws IOException {
        socket = new Socket(Config.ip, Config.port);

        is = socket.getInputStream();
        os = socket.getOutputStream();
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
        byte[] bytes= new byte[byteList.size()];
        for(int i = 0; i < byteList.size(); i++) {
            bytes[i] = byteList.get(i).byteValue();
        }

        ByteBuffer buffer = ByteBuffer.wrap(bytes);

        while(true) {
            int remaining = buffer.remaining();
            if(remaining < 2 * Integer.BYTES) {
                break;
            }

            int size = buffer.getInt() - 2 * Integer.BYTES;
            if(size <= 0) {
                continue;
                // invalid message
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

            messageQueue.add(new Message(type, data));
        }
    }

    public void send(Message msg) throws IOException {
        os.write(msg.serialize());
    }

    public Message getMessage(MessageType... types) {
        while(true) {
            if(messageQueue.isEmpty()) {
                continue;
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
