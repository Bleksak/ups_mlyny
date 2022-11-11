package model;

import java.net.Socket;

public record Message(Socket socket, MessageType type, byte[] data) {
    // public Message(Socket socket, byte[] data) {
    //     m_socket = socket;
    //     m_data = data;
    // }
}
