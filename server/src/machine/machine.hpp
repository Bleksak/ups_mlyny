#pragma once
#include "state.hpp"
#include <iostream>
#include <utility>

class Socket;
class Machine {
    public:
        Machine(Socket* sock) : m_socket(sock) {
            m_state = new Init(this);
        }
        
        Machine(Machine&& o) : m_socket(o.m_socket) {
            m_state = o.m_state;
            o.m_state = nullptr;
        }
        
        Machine& operator=(Machine&& o);
        
        ~Machine() {
            if(m_state != nullptr) {
                delete m_state;
            }
        }
        
        void update_socket(Socket* sock);
        
        void transition(AbstractState* state) {
            delete m_state;
            m_state = state;
        }
        
        void handle_message(RECV_TYPE RECV_VALUE) {
            m_state->handle_message(RECV_VALUE);
        }
        
        Socket* socket() {
            return m_socket;
        }
        
    private: 
        AbstractState* m_state;
        Socket* m_socket;
};
