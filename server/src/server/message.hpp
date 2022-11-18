#pragma once

#include <vector>

class Socket;
class Message {
    public:
        enum Type {
            OK = 0,
            NOK,
            PLAYER_INIT,
        };
        
        Message(int socket, Type type, std::vector<char> data) : m_socket(socket), m_type(type), m_data(data) {}
        
        int socket() {
            return m_socket;
        }
        
        Type type() {
            return m_type;
        }
        
        std::vector<char>& data() {
            return m_data;
        }
    
    private:
        int m_socket;
        Type m_type;
        std::vector<char> m_data;
};