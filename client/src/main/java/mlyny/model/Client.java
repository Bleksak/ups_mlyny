package mlyny.model;

import java.io.*;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.SocketChannel;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Set;

public class Client extends Thread implements Closeable {

    private final Selector m_selector;
    private SocketChannel m_socket = null;

    // private final int timeout = 5000;
    private final String m_address;
    private final int m_port;

    private final Sender m_sender = new Sender(this);
    private final Receiver m_receiver = new Receiver(this);

    private boolean m_running = false;

    private static Client m_instance;

    private Client(String address, int port) throws IOException {
        m_address = address;
        m_port = port;
        m_selector = Selector.open();
        initialize();
    }

    private void initialize() throws IOException {
        if(m_socket == null) {
            m_socket = SocketChannel.open();
            m_socket.connect(new InetSocketAddress(m_address, m_port));
            m_socket.configureBlocking(false);
            m_socket.register(m_selector, SelectionKey.OP_READ);
        }

        System.out.println("init ok!");
    }

    public boolean running() {
        return m_running;
    }

    public boolean connect() throws IOException {
        if(m_socket.isConnected()) {
            return true;
        }

        return m_socket.finishConnect();
    }

    public static Client getInstance() throws IOException {
        if(m_instance == null) {
            m_instance = new Client(Config.address, Config.port);
        }

        return m_instance;
    }

    public void sendMessage(Message message) throws IOException {
        m_socket.write(message.buffer());
    }
    
    public void parseMessages() throws IOException {
        System.out.println("parsing messages!");

        ByteBuffer bbuffer = ByteBuffer.allocate(8192);
        List<Byte> byteList = new ArrayList<Byte>();

        while(true) {
            bbuffer.clear();
            int read = m_socket.read(bbuffer);

            if(read == 0) {
                break;
            }

            byte[] bytes = bbuffer.slice(0, read).array();
            for(int i = 0; i < bytes.length; ++i) {
                byteList.add(bytes[i]);
            }
        }

        byte[] bytes = new byte[byteList.size()];
        for(int i = 0; i < byteList.size(); ++i) {
            bytes[i] = byteList.get(i);
        }

        int messageStart = 0;
        final int msg_offset = 2 * Integer.BYTES;

        while(messageStart + Integer.BYTES < bytes.length) {
            ByteBuffer buffer = ByteBuffer.wrap(Arrays.copyOfRange(bytes, messageStart, messageStart + Integer.BYTES));
            int messageLength = buffer.getInt();
            buffer = ByteBuffer.wrap(Arrays.copyOfRange(bytes, messageStart + Integer.BYTES, messageStart + msg_offset));

            int messageTypeInt = buffer.getInt();
            
            if(bytes.length < messageLength) {
                System.out.println("Invalid buffer length!");
                return;
            }

            MessageType messageType = MessageType.valueOf(messageTypeInt);

            byte[] messageData = Arrays.copyOfRange(bytes, msg_offset, messageLength - msg_offset);
            m_receiver.pushMessage(new Message(m_socket.socket(), messageType, messageData));

            messageStart += messageLength;
        }
    }

    public void run() {
        m_running = true;

        while(true) {
            if(!m_running) {
                try {
                    Thread.sleep(30);
                } catch (InterruptedException e) {}
            }

            try {
                m_selector.select();
                Set<SelectionKey> keys = m_selector.selectedKeys();
                
                for(SelectionKey key : keys) {
                    if(!key.isReadable()) {
                        continue;
                    }

                    parseMessages();
                }

                keys.clear();

            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }

    public void stopThread() {
        m_running = false;
    }

    public void startThread() {
        m_running = true;
    }

    @Override
    public void close() {
        m_running = false;

        try {
            if(m_socket != null) {
                m_socket.close();
            }
        } catch(IOException ex) {}
    }

    public void ping() {
        m_sender.pushMessage(new Message(m_socket.socket(), MessageType.PING, null));
    }

    public void createGameRequest(String username) {
        m_sender.pushMessage(new Message(m_socket.socket(), MessageType.PLAYER_INIT_CREATE, username.getBytes(StandardCharsets.US_ASCII)));
    }

    public void joinGameRequest(String username) {

    }

}
