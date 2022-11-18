#include "state.hpp"
#include "../socket.hpp"
#include "machine.hpp"

#define HANDLER(name) void name::handle_message(RecvMessage& message) 

State::State(Socket* sock) : m_sock(sock) {}
State::State(StateMachine* machine, Socket* sock) : m_machine(machine), m_sock(sock) {}

void State::set_machine(StateMachine* machine) {
    m_machine = machine;
}

HANDLER(Init) {
    // accept only JOIN and CREATE messages
}

HANDLER(InLobby) {
    // accept only: GET_CONNECTED_PLAYERS_CNT message
}

HANDLER(InGamePutTurn) {
    // accept only: PUT_STONE
}

// put stone
HANDLER(InGamePut) {
    // dont accept anything(waiting state)
}

HANDLER(InGameTurn) {
    // accept: MOVE_STONE
}

// move stone
HANDLER(InGame) {
    // accept only: nothing (waiting state)
}

// steaaal
HANDLER(InGameTakeTurn) {
    // accept: STEAL_STONE
}

HANDLER(InGameTake) {
    // accept only: nothing (waiting state)
}
