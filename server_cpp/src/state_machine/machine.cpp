#include "machine.hpp"

StateMachine::StateMachine(State state) : m_current_state(state) {}

void StateMachine::handle_message(RecvMessage &message) {
    m_current_state.handle_message(message);
}

void StateMachine::set_state(State&& state) {
    m_current_state = std::move(state);
}
