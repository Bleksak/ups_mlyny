package model;

import java.io.DataOutputStream;
import java.io.IOException;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.SocketChannel;

public class Client {

    private Socket socket;
    private SocketChannel channel = SocketChannel.open();
    private Selector selector = Selector.open();
    private DataOutputStream outputStream;

    public Client(String address, int port) throws IOException {
        channel.configureBlocking(false);
        channel.register(selector, SelectionKey.OP_CONNECT | SelectionKey.OP_READ | SelectionKey.OP_WRITE);
        channel.connect(new InetSocketAddress(InetAddress.getByName(address), port));
    }


    private void sendRequest() {

    }

    private void receiveData() {

    }
}
