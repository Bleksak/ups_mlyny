#pragma once
#include <iostream>
#include <string>
#include <unistd.h>

class Socket {
    public:
        Socket(int socket, std::string identifier) : m_socket(socket), m_identifier(identifier) {}
        // auto bind(std::string identifier) -> void {
        //     m_identifier = std::move(identifier);
        // }
        
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
            std::cout << "closing socket\n";
            
            if(m_socket != -1) {
                close(m_socket);
            }
        }
        
        auto socket() const -> int {
            return m_socket;
        }
        
        auto identifier() const -> const std::string& {
            return m_identifier;
        }
    
    private:
        int m_socket;
        std::string m_identifier;
};
