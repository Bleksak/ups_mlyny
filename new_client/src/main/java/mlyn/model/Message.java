package mlyn.model;

import java.nio.ByteBuffer;

public record Message(MessageType type, byte[] data) {
    public byte[] serialize() {
        int sizeInt = 2 * Integer.BYTES + (data == null ? 0 : data.length);

        if(data == null) {
            return ByteBuffer.allocate(sizeInt).putInt(sizeInt).putInt(type.value()).array();
        }

        return ByteBuffer.allocate(sizeInt).putInt(sizeInt).putInt(type.value()).put(data).array();
    }
}
