#pragma once
#include <string>

class Socket {
    public:
        Socket(int socket, std::string identifier) : m_socket(socket), m_identifier(identifier) {}
        // auto bind(std::string identifier) -> void {
        //     m_identifier = std::move(identifier);
        // }
        
        auto socket() const -> int {
            return m_socket;
        }
        
        auto identifier() const -> const std::string& {
            return m_identifier;
        }
    
    private:
        const std::string m_identifier;
        const int m_socket;
};
