package mlyn.model;


public class Machine {
    public enum State {
        CONNECTED(0),
        LOBBY(1),
        GAME_PUT(2),
        GAME_TAKE(3),
        GAME_MOVE(4),
        GAME_OVER(5);

        int value;
        
        State(int value) {
            this.value = value;
        }

        public static State valueOf(int value) {
            switch(value) {
                case 0: return CONNECTED;
                case 1: return LOBBY;
                case 2: return GAME_PUT;
                case 3: return GAME_TAKE;
                case 4: return GAME_MOVE;
                case 5: return GAME_OVER;
            }

            return null;
        }
    }

    private State state;

    public Machine() {
        state = State.CONNECTED;
    }

    public Machine(State state) {
        setState(state);
    }

    public void setState(State state) {
        this.state = state;
    }

    public State getState() {
        return state;
    }
}
