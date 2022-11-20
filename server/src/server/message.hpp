#pragma once

#include <netinet/in.h>
#include <vector>
#include <cstddef>
#include <cstdint>

class Socket;
class Message {
    public:
        enum Type {
            OK = 0,
            NOK ,
            INIT,
            CREATE,
            JOIN,
            PUT,
            MOVE,
            TAKE,
        };
        
        Message(Message&& o) {
            m_socket = o.m_socket;
            m_type = o.m_type;
            m_data = std::move(o.m_data);
        }
        
        Message(int socket, Type type, std::vector<char> data) : m_socket(socket), m_type(type), m_data(std::move(data)) {}
        
        Message(int socket, Type type, size_t size, void* data) : m_socket(socket), m_type(type) {
            uint32_t type_uint = static_cast<uint32_t>(htonl(type));
            uint32_t msg_size = htonl(size + sizeof(size) + sizeof(type_uint));
            
            m_data.insert(m_data.end(), sizeof(size), msg_size);
            m_data.insert(m_data.end(), sizeof(type_uint), type_uint);
            
            char* char_data = reinterpret_cast<char*>(data);
            std::copy(char_data, char_data + size, m_data.end());
        }
        
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
