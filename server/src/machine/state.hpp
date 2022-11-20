#pragma once

#include "../server/message.hpp"

// #define RECV_TYPE 
// #define RECV_VALUE
#define RECV_TYPE Message&
#define RECV_VALUE message
#define MAKE_STATE(name) class name : public AbstractState { public: name (Machine* machine) : AbstractState(machine) {} void handle_message(RECV_TYPE RECV_VALUE); }

class Machine;
class AbstractState {
    public:
        AbstractState(Machine* machine) : m_machine(machine) {}
        virtual void handle_message(RECV_TYPE RECV_VALUE) = 0;
        virtual ~AbstractState() {};
    protected:
        Machine* m_machine;
};

MAKE_STATE(Init);
MAKE_STATE(Lobby);
MAKE_STATE(GamePut);
MAKE_STATE(GameMove);
MAKE_STATE(GameTake);
MAKE_STATE(GameOver);
