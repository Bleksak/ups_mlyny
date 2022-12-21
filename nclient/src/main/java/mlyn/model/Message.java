package mlyn.model;

import java.nio.ByteBuffer;

public class Message {

    public MessageType _type;
    public byte[] _data;

    public Message(MessageType type, byte[] data) {
        this._type = type;
        this._data = data;
    }

    public MessageType type() {
        return _type;
    }

    public byte[] data() {
        return _data;
    }

    public byte[] serialize() {
        int sizeInt = 2 * Integer.BYTES + (_data == null ? 0 : _data.length);

        if(_data == null) {
            return ByteBuffer.allocate(sizeInt).putInt(sizeInt).putInt(_type.value()).array();
        }

        return ByteBuffer.allocate(sizeInt).putInt(sizeInt).putInt(_type.value()).put(_data).array();
    }
}
