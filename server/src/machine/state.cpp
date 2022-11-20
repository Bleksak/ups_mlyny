#include "state.hpp"
#include <sys/socket.h>
#include "machine.hpp"
#include "../server/socket.hpp"
#include "../server/server.hpp"

#define HANDLER(name) void name::handle_message(RECV_TYPE RECV_VALUE)

HANDLER(Init) {
    if(message.type() != Message::INIT) {
        m_machine->socket()->server()->sender().push_message(Message(message.socket(), Message::Type::NOK, 0, nullptr));
    }
}

HANDLER(Lobby) {
    (void) message;
}

HANDLER(GamePut) {
    (void) message;
}

HANDLER(GameMove) {
    (void) message;
}

HANDLER(GameTake) {
    (void) message;
}

HANDLER(GameOver) {
    (void) message;
}
