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
                // case 0: return CONNECTED;
                // case 1: return LOBBY;
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

    public boolean validColor(char c) {
        if(c == '0' || c == '1' || c == '2') return true;
        return false;
    }

    public boolean validateMessage(Message msg) {
        switch(state) {
            case CONNECTED:
            case LOBBY:
                if(msg.type() == MessageType.JOINED) {
                    return true;
                } 
                
                if(msg.type() == MessageType.NOK) {
                    if(msg.data().length == 1) {
                        return true;
                    }
                    return false;
                }

                if(msg.type() != MessageType.READY) {
                    return false;
                }

                if(msg.data().length != 4) {
                    return false;
                }

                int state = Integer.parseInt(msg.data()[0]);
                if(Machine.State.valueOf(state) == null) {
                    System.out.println("invalid state");
                    return false;
                }
                
                if(msg.data()[1].length() != 1 || !validColor(msg.data()[1].charAt(0))) {
                    System.out.println("invalid color");
                    return false;
                }

                if(msg.data()[2].length() != 24) {
                    System.out.println("bad board size");
                    return false;
                }

                for(int i = 0; i < msg.data()[2].length(); ++i) {
                    if(!validColor(msg.data()[2].charAt(i))) {
                        System.out.println("invalid color on board");
                        return false;
                    }
                }

                if(msg.data()[3].isEmpty()) {
                    System.out.println("opponent name empty");
                    return false;
                }

                try {
                } catch(NumberFormatException ex) {
                    return false;
                }

                return true;
            case GAME_MOVE:
                if(msg.type() != MessageType.OVER && msg.type() != MessageType.DISCONNECT && msg.type() != MessageType.OK && msg.type() != MessageType.NOK && msg.type() != MessageType.CRASH && msg.type() != MessageType.STATE) {
                    System.out.println("move accepts only disconnect/nok/ok messages");
                    return false;
                }

                if(msg.type() == MessageType.STATE) {
                    System.out.println("STATE CHANGE????");
                    if(msg.data().length != 1) {
                        return false;
                    }
                    try {
                    int new_state = Integer.parseInt(msg.data()[0]);
                    if(Machine.State.valueOf(new_state) == null) {
                        return false;
                    }
                    } catch(NumberFormatException e) {
                        return false;
                    }

                    return true;
                }

                if(msg.type() == MessageType.NOK) {
                    if(msg.data()[0].isEmpty()) {
                        System.out.println("no message");
                        return false;
                    }
                    return true;
                }

                if(msg.type() == MessageType.OK) {
                    return true;
                }

                return false;
            case GAME_MOVE_OPP:
                if(msg.type() == MessageType.OVER || msg.type() == MessageType.DISCONNECT || msg.type() == MessageType.CRASH || msg.type() == MessageType.STATE) {
                    if(msg.type() == MessageType.STATE) {
                        if(msg.data().length != 1) {
                            return false;
                        }
                        try {
                        int new_state = Integer.parseInt(msg.data()[0]);
                        if(Machine.State.valueOf(new_state) == null) {
                            return false;
                        }
                        } catch(NumberFormatException e) {
                            return false;
                        }
                    }
                    return true;
                }

                try {

                if(msg.type() != MessageType.PLAYER_MV) {
                    return false;
                }
                if(msg.data().length != 2) {
                    System.out.println("bad arguments");
                    return false;
                }

                int index1 = Integer.parseInt(msg.data()[0]);
                int index2 = Integer.parseInt(msg.data()[1]);

                if(index1 < 0 || index1 >= 24) {
                    System.out.println("bad index");
                    return false;
                }

                if(index2 < 0 || index2 >= 24) {
                    System.out.println("bad index");
                    return false;
                }

                if(index1 == index2) {
                    System.out.println("bad index");
                    return false;
                }

                } catch(NumberFormatException ex) {
                    return false;
                }

                return true;
            case GAME_OVER:
                return true;
            case GAME_PUT:
                if(msg.type() == MessageType.DISCONNECT || msg.type() == MessageType.CRASH) {
                    return true;
                }

                if(msg.type() == MessageType.STATE) {
                    if(msg.data().length != 1) {
                        return false;
                    }
                    try {
                    int new_state = Integer.parseInt(msg.data()[0]);
                    if(Machine.State.valueOf(new_state) == null) {
                        return false;
                    }
                    } catch(NumberFormatException e) {
                        return false;
                    }
                    return true;
                }

                if(msg.type() == MessageType.NOK) {
                    if(msg.data().length != 1) {
                        return false;
                    }

                    if(msg.data()[0].isEmpty()) {
                        return false;
                    }
                    return true;
                }

                if(msg.type() == MessageType.OK) {
                    return true;
                }

                return false;
            case GAME_PUT_OPP:
                if(msg.type() == MessageType.DISCONNECT || msg.type() == MessageType.CRASH) {
                    return true;
                }

                if(msg.type() == MessageType.STATE) {
                    if(msg.data().length != 1) {
                        return false;
                    }
                    try {
                    int new_state = Integer.parseInt(msg.data()[0]);
                    if(Machine.State.valueOf(new_state) == null) {
                        return false;
                    }
                    } catch(NumberFormatException e) {
                        return false;
                    }

                    return true;
                }

                if(msg.type() != MessageType.PLAYER_PUT) {
                    return false;
                }

                if(msg.data().length != 1) {
                    System.out.println("bad arguments");
                    return false;
                }
                try {

                int index1 = Integer.parseInt(msg.data()[0]);

                if(index1 < 0 || index1 >= 24) {
                    System.out.println("bad index");
                    return false;
                }

                } catch(NumberFormatException ex) {
                    return false;
                }

                return true;

            case GAME_TAKE:
                if(msg.type() == MessageType.OVER || msg.type() == MessageType.DISCONNECT || msg.type() == MessageType.CRASH) {
                    return true;
                }

                if(msg.type() == MessageType.STATE) {
                    if(msg.data().length != 1) {
                        return false;
                    }
                    try {
                    int new_state = Integer.parseInt(msg.data()[0]);
                    if(Machine.State.valueOf(new_state) == null) {
                        return false;
                    }
                    } catch(NumberFormatException e) {
                        return false;
                    }
                    return true;
                }

                if(msg.type() == MessageType.NOK) {
                    if(msg.data().length != 1) {
                        return false;
                    }
                    if(msg.data()[0].isEmpty()) {
                        System.out.println("no message");
                        return false;
                    }
                    return true;
                }

                if(msg.type() == MessageType.OK) {
                    return true;
                }

                return false;
            case GAME_TAKE_OPP:
                if(msg.type() == MessageType.OVER || msg.type() == MessageType.DISCONNECT || msg.type() == MessageType.CRASH) {
                    return true;
                }

                if(msg.type() == MessageType.STATE) {
                    if(msg.data().length != 1) {
                        return false;
                    }
                    try {
                    int new_state = Integer.parseInt(msg.data()[0]);
                    if(Machine.State.valueOf(new_state) == null) {
                        return false;
                    }
                    } catch(NumberFormatException e) {
                        return false;
                    }

                    return true;
                }

                if(msg.type() != MessageType.PLAYER_TAKE) {
                    return false;
                }

                if(msg.data().length != 1) {
                    System.out.println("bad arguments");
                    return false;
                }
                try {

                int index1 = Integer.parseInt(msg.data()[0]);

                if(index1 < 0 || index1 >= 24) {
                    System.out.println("bad index");
                    return false;
                }

                } catch(NumberFormatException ex) {
                    return false;
                }

                return true;
        }

        return false;
    }

    public void handleMessage(Message msg) {

    }

}
