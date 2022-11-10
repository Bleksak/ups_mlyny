package model;

import java.io.*;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.channels.ClosedChannelException;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.SocketChannel;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Set;

public class Client extends Thread implements Closeable {

    private final byte NEWLINE = 13;

    private final Selector m_selector;
    private SocketChannel m_socket = null;

    // private final int timeout = 5000;
    private final String m_address;
    private final int m_port;

    private final Sender m_sender = new Sender(this);
    private final Receiver m_receiver = new Receiver(this);

    private boolean m_running = true;

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

    public void sendMessage(Message message) {
        
    }
    
    public void parseMessages() throws IOException {
        Socket socket = m_socket.socket();
        DataInputStream is = new DataInputStream(socket.getInputStream());

        byte[] bytes = is.readAllBytes();
        int messageStart = 0;
        int newlineChar = 0;

        while(true) {

            ByteBuffer buffer = ByteBuffer.allocate(Long.BYTES);
            buffer.put(Arrays.copyOfRange(bytes, messageStart, messageStart + Long.BYTES));
            buffer.flip();
            long messageLength = buffer.getLong();
            
            if(bytes.length < messageLength) {
                // invalid buffer length (received data too short)
                return;
            }

            for(int i = messageStart + Long.BYTES; i <= bytes.length; ++i) {
                if(i == bytes.length || bytes[i] == NEWLINE) {
                    newlineChar = i;
                    break;
                }
            }

            if(newlineChar == bytes.length) {
                // MESSAGE END (MB INVALID MESSAGE?)
                return;
            }

            // read messageStart...newlineChar
            String messageTypeStr = new String(Arrays.copyOfRange(bytes, messageStart + Long.BYTES, newlineChar), StandardCharsets.US_ASCII);
            MessageType messageType = MessageType.getType(messageTypeStr);

            if(messageType == MessageType.INVALID) {
                // invalid message
                return;
            }

            // TODO: the cast may be unsafe
            byte[] messageData = Arrays.copyOfRange(bytes, newlineChar+1, (int) messageLength);
            // m_receiver.pushMessage(new Message(socket, messageType, messageData));
        }
    }

    public void run() {

        while(true) {

            // try {
            //     selector.select();
            //     Set<SelectionKey> keys = selector.selectedKeys();

            //     for(SelectionKey key : keys) {
            //         if(key.isConnectable()) {
            //             System.out.println("Connectable!");
            //         }

            //         if(key.isReadable()) {

            //         }
            //     }

            // } catch (IOException e) {
            //     e.printStackTrace();
            // }

        }
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
}
