package mlyny.model;

import java.util.HashMap;

public enum MessageType {
    OK(0),
    NOK(1),
    PLAYER_INIT_CREATE(2),
    PLAYER_INIT_JOIN(3),
    READY(4),
    PLAYER_PUT(5),
    PLAYER_MV(6),
    PLAYER_TAKE(7),
    OVER(8),
    PING(9),
    PONG(10),
    PLAYER_INIT_USERNAME_INVALID(11),
    PLAYER_INIT_USERNAME_USED(12);

    private final int m_value;
    private static HashMap<Integer, MessageType> m_reverseMap = new HashMap<>();

    MessageType(int value) {
        m_value = value;
    }

    static {
        for (MessageType type : MessageType.values()) {
            m_reverseMap.put(type.m_value, type);
        }
    }

    public int value() {
        return m_value;
    }

    public static MessageType valueOf(int value) {
        return m_reverseMap.getOrDefault(value, NOK);
    }
}
