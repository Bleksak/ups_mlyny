#pragma once

class StateMachine;
class Socket;

#include "../message.hpp"

#define MAKE_STATE(name) class name : public State { public: name(Socket* sock) : State(sock) {} name(StateMachine* machine, Socket* sock) : State(machine, sock) {} void handle_message(RecvMessage& message) override; }

class State {
    protected:
        StateMachine* m_machine;
        Socket* m_sock;
    
    public:
        State(Socket* sock);
        State(StateMachine* machine, Socket* sock);
        void set_machine(StateMachine* machine);
        
        virtual void handle_message(RecvMessage& message);
};


MAKE_STATE(Init);
MAKE_STATE(InLobby);

// put stone
MAKE_STATE(InGamePut);
MAKE_STATE(InGamePutTurn);

// move stone
MAKE_STATE(InGame);
MAKE_STATE(InGameTurn);

// steaaal
MAKE_STATE(InGameTake);
MAKE_STATE(InGameTakeTurn);

