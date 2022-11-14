package mlyny.model;

import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;

public record Message(Socket socket, MessageType type, byte[] data) {
    public ByteBuffer buffer() {

        int dataLength = data == null ? 0 : data.length;

        byte[] messageType = type.message().getBytes(StandardCharsets.US_ASCII);
        int length = Long.BYTES + dataLength + messageType.length;

        byte[] buf = new byte[length];
        ByteBuffer intBuffer = ByteBuffer.allocate(Integer.BYTES);
        intBuffer.putInt(Integer.reverseBytes(length));
        
        for(int i = 0; i < Integer.BYTES; ++i) {
            buf[i] = intBuffer.get(i);
        }

        for(int i = 0; i < messageType.length; ++i) {
            buf[Integer.BYTES + i] = messageType[i];
        }

        for(int i = 0; i < dataLength; ++i) {
            buf[Integer.BYTES + messageType.length + i] = data[i];
        }

        return ByteBuffer.wrap(buf);
    }
}
