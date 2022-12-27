package mlyn.model;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;

public class Message {
    public MessageType _type;
    public String[] _data;

    public Message(MessageType type, String... data) {
        this._type = type;
        this._data = data;
    }

    public MessageType type() {
        return _type;
    }

    public String[] data() {
        return _data;
    }

    public byte[] serialize() {
        byte[] serializedMessageType = (type().value() + ';').getBytes(StandardCharsets.UTF_8);

        String data = String.join(";", _data);
        int len = data.length();

        int sizeInt = serializedMessageType.length + len;
        String sizeString = Integer.toString(sizeInt) + ';';

        // if(_data != null) {

        ByteBuffer buffer = ByteBuffer.allocate(sizeInt + sizeString.length());
        // System.out.println(sizeInt + len + sizeString.length());
        buffer.put(sizeString.getBytes(StandardCharsets.UTF_8));
        buffer.put(serializedMessageType);
        buffer.put(data.getBytes(StandardCharsets.UTF_8));

        return buffer.array();
        // }

        // return ByteBuffer.allocate(sizeInt+sizeString.length()).put(sizeString.getBytes(StandardCharsets.UTF_8)).put(serializedMessageType).array();
    }
}
