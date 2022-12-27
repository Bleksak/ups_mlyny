package mlyn.model;

import java.util.HashMap;

public enum MessageType {
    INVALID("INVALID"),
    OK("OK"),
    NOK("NOK"),
    PLAYER_INIT_CREATE("CREATE"),
    PLAYER_INIT_JOIN("JOIN"),
    READY("READY"),
    PLAYER_PUT("PUT"),
    PLAYER_TAKE("TAKE"),
    PLAYER_MV("MOVE"),
    OVER("OVER"),
    PING("PING"),
    PONG("PONG"),
    JOINED("JOINED"),
    DISCONNECT("DISCONNECT"),
    STATE("STATE"),
    CRASH("CRASH");

    private final String m_value;
    private static HashMap<String, MessageType> m_reverseMap = new HashMap<>();

    MessageType(String value) {
        m_value = value;
    }

    static {
        for (MessageType type : MessageType.values()) {
            m_reverseMap.put(type.m_value, type);
        }
    }

    public String value() {
        return m_value;
    }

    public static MessageType valueOfString(String value) {
        return m_reverseMap.getOrDefault(value, INVALID);
    }
}
