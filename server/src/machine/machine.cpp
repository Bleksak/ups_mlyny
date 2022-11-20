#include "machine.hpp"
#include "../server/socket.hpp"
#include <iostream>

Machine& Machine::operator=(Machine&& o) {
    m_socket = o.m_socket;
    m_state = o.m_state;
    o.m_state = nullptr;
            
    return *this;
}
void Machine::update_socket(Socket* sock) {
    std::cout << "updating SOCKET from: " << m_socket << " to: " << sock << std::endl;
    m_socket = sock;
}
