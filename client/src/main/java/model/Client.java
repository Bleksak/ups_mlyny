package model;

import java.io.*;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.SocketChannel;

public class Client implements Closeable {
    private Socket socket;
    private DataInputStream inputStream;
    private DataOutputStream outputStream;

    private final int timeout = 5000;
    private final String address;
    private final int port;

    private static Client instance;

    private Client(String address, int port) throws IOException {
        this.address = address;
        this.port = port;
        tryInit();
//        socket.setReuseAddress(true);
    }

    private void tryInit() throws IOException {
        if(socket == null || !socket.isConnected()) {
            socket = new Socket(address, port);
        }

        System.out.println("init ok!");
    }

    public static Client getInstance() throws IOException {
        if(instance == null) {
            instance = new Client(Config.address, Config.port);
        }

        return instance;
    }

    public void beginRead() throws IOException {
        inputStream = new DataInputStream(socket.getInputStream());
    }

    public void beginWrite() throws IOException {
        outputStream = new DataOutputStream(socket.getOutputStream());
    }

    public void endRead() throws IOException {
        if(inputStream != null) {
            inputStream.close();
        }
    }

    public void endWrite() throws IOException {
        if(inputStream != null) {
            outputStream.close();
        }
    }

    public void writeInt(int value) throws IOException {
        System.out.println("sending " + value);
        outputStream.writeInt(Integer.reverseBytes(value));
    }

    public void writeLong(long value) throws IOException {
        System.out.println("sending " + value);
        outputStream.writeLong(Long.reverseBytes(value));
    }

    public int readInt() throws IOException {
        int value = Integer.reverseBytes(inputStream.readInt());
        System.out.println("reading " + value);
        return value;
    }

    public long readLong() throws IOException {
        long value = Long.reverseBytes(inputStream.readLong());
        System.out.println("reading " + value);
        return value;
    }

    @Override
    public void close() {
        try {
            socket.close();
        } catch (Exception ignored) {}
    }
}
