package mlyn.model;


public class Machine {
    public enum State {
        LOBBY,
        GAME_PUT,
        GAME_TAKE,
        GAME_MOVE,
        GAME_OVER,

    }

    private State state;

    public Machine(State state) {
        this.state = state;
    }
}
