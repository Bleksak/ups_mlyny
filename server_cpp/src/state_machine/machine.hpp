#pragma once

class StateMachine;

#include "state.hpp"
#include "../message.hpp"

class StateMachine {
    public:
        StateMachine(State state);
        void handle_message(RecvMessage& message);
        void set_state(State&& state);
        
    private:
        State m_current_state;
};
