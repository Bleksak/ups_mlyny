#include "machine.hpp"
#include "../server/socket.hpp"

Machine& Machine::operator=(Machine&& o) {
    m_socket = std::move(o.m_socket);
    m_state = o.m_state;
    o.m_state = nullptr;
            
    return *this;
}
void Machine::update_socket(Socket& sock) {
    m_socket = std::move(sock);
}
