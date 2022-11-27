package mlyny.model;

import java.net.Socket;
import java.nio.ByteBuffer;

public record Message(Socket socket, MessageType type, byte[] data) {
    public ByteBuffer buffer() {
        int dataLength = data == null ? 0 : data.length;

        int type_int = type.value();

        int length = 2 * Integer.BYTES + dataLength;
        System.out.println("message length: " + length);

        byte[] buf = new byte[length];
        ByteBuffer intBuffer = ByteBuffer.allocate(Integer.BYTES);
        intBuffer.putInt(length);
        
        for(int i = 0; i < Integer.BYTES; ++i) {
            buf[i] = intBuffer.get(i);
        }

        intBuffer.clear();
        intBuffer.putInt(type_int);

        for(int i = 0; i < Integer.BYTES; ++i) {
            buf[Integer.BYTES + i] = intBuffer.get(i);
        }

        for(int i = 0; i < dataLength; ++i) {
            buf[2 * Integer.BYTES + i] = data[i];
        }

        return ByteBuffer.wrap(buf);
    }
}
