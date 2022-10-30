package model;

public enum ServerMessage {
    INVALID(0),
    CREATE_GAME(1),
    JOIN_GAME(2),
    PUT_STONE(3),
    MOVE_STONE(4),
    CONNECT(5),
    DISCONNECT(6),
    PONG(7);

    public final int value;

    ServerMessage(int value) {
        this.value = value;
    }
}
