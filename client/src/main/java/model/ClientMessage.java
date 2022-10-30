package model;

public enum ClientMessage {
    OK(0),
    FAIL(1),
    GAME_CREATED(2),
    GAME_JOINED(3),
    GAME_OPPONENT_JOINED(4),
    PING(5);

    public final int value;

    private ClientMessage(int value) {
        this.value = value;
    }
}
