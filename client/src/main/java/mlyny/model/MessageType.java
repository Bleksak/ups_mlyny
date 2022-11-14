package mlyny.model;

import java.util.HashMap;

public enum MessageType {
    INVALID(""),
    OK("LIFE IS GOOD\n"),
    NOK("LIFE IS BAD\n"),
    PLAYER_INIT("TELL ME WHO YOU ARE\n"),
    PLAYER_INIT_CREATE("BY YOUR HAND ALL THINGS WERE MADE... EVEN ME\n"),
    PLAYER_INIT_JOIN(""),
    PLAYER_PUT("SIT DOWN\n"),
    PLAYER_MV("IM GONNA DO WHATS CALLED A PRO-GAMER MOVE\n"),
    PLAYER_TAKE("NIGGAS GONNA ROB\n"),
    PING("KNOCK KNOCK\n"),
    PONG("WHOS THERE?\n"),
    PLAYER_INIT_USERNAME_INVALID("YOUR ARGUMENT IS INVALID\n"),
    PLAYER_INIT_USERNAME_USED("IF I LICK IT ITS MINE\n");

    private final String m_message;

    MessageType(String message) {
        m_message = message;
    }

    public String message() {
        return m_message;
    }

    private static HashMap<String, MessageType> m_reverseMap = new HashMap<>();
    static {
        m_reverseMap.put("", INVALID);
        m_reverseMap.put("LIFE IS GOOD\n", OK);
        m_reverseMap.put("LIFE IS BAD\n", NOK);
        m_reverseMap.put("TELL ME WHO YOU ARE\n", PLAYER_INIT);
        m_reverseMap.put("BY YOUR HAND ALL THINGS WERE MADE... EVEN ME\n", PLAYER_INIT_CREATE);
        m_reverseMap.put("I AM TELLING YOU WHO I AM\n", PLAYER_INIT_JOIN);
        m_reverseMap.put("SIT DOWN\n", PLAYER_PUT);
        m_reverseMap.put("IM GONNA DO WHATS CALLED A PRO-GAMER MOVE\n", PLAYER_MV);
        m_reverseMap.put("NIGGAS GONNA ROB\n", PLAYER_TAKE);
        m_reverseMap.put("KNOCK KNOCK\n", PING);
        m_reverseMap.put("WHOS THERE?\n", PONG);
        m_reverseMap.put("YOUR ARGUMENT IS INVALID\n", PLAYER_INIT_USERNAME_INVALID);
        m_reverseMap.put("IF I LICK IT ITS MINE\n", PLAYER_INIT_USERNAME_USED);
    }

    public static MessageType getType(String str) {
        return m_reverseMap.getOrDefault(str, INVALID);
    }
}
