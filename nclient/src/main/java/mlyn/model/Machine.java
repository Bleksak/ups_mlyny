package mlyn.model;


public class Machine {
    public enum State {
        CONNECTED(0),
        LOBBY(1),
        GAME_PUT(2),
        GAME_PUT_OPP(3),
        GAME_TAKE(4),
        GAME_TAKE_OPP(5),
        GAME_MOVE(6),
        GAME_MOVE_OPP(7),
        GAME_OVER(8);

        int value;
        
        State(int value) {
            this.value = value;
        }

        public static State valueOf(int value) {
            switch(value) {
                case 0: return CONNECTED;
                case 1: return LOBBY;
                case 2: return GAME_PUT;
                case 3: return GAME_PUT_OPP;
                case 4: return GAME_TAKE;
                case 5: return GAME_TAKE_OPP;
                case 6: return GAME_MOVE;
                case 7: return GAME_MOVE_OPP;
                case 8: return GAME_OVER;
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
