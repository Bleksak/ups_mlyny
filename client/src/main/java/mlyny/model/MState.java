package mlyny.model;

public enum MState {
    INIT,
    LOBBY,
    GAME_PUT,
    GAME_TAKE,
    GAME_MOVE,
    GAME_OVER,
    BULLSHIT;

    public static MState fromInt(int value) {
        switch(value) {
            case 0: return INIT;
            case 1: return LOBBY;
            case 2: return GAME_PUT;
            case 3: return GAME_TAKE;
            case 4: return GAME_MOVE;
            case 5: return GAME_OVER;
        }

        return BULLSHIT;
    }

}
