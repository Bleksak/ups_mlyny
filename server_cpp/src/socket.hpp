#pragma once
#include "state_machine/machine.hpp"
#include <iostream>
#include <string>
#include <unistd.h>

class Socket {
    public:
        Socket(int socket) : m_socket(socket), m_machine() {
            State s = Init(this);
            StateMachine m(s);
            m_machine = m;
        }
        
        Socket(Socket&& other) {
            m_socket = other.m_socket;
            std::swap(m_identifier, other.m_identifier);
            other.m_socket = -1;
        }
        
        Socket& operator=(Socket&& other) {
            m_socket = other.m_socket;
            std::swap(m_identifier, other.m_identifier);
            other.m_socket = -1;
            
            return *this;
        }
        
        ~Socket() {
            if(m_socket != -1) {
                close(m_socket);
            }
        }
        
        auto bind(std::string identifier) -> void {
            m_identifier = std::move(identifier);
        }
        
        auto socket() const -> int {
            return m_socket;
        }
        
        auto identifier() const -> const std::string& {
            return m_identifier;
        }
    
    private:
        StateMachine m_machine;
        int m_socket;
        std::string m_identifier;
};
