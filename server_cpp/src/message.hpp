#pragma once

#include <cstring>
#include <iostream>
#include <vector>
#include <sys/socket.h>

class Message {
    public:
        Message(int socket, size_t size, char* data) : m_data(new char[size]), m_size(size), m_socket(socket) {
            std::memcpy(this->m_data, data, size);
        }
        
        // copy ctor
        Message(const Message& msg) : Message(msg.m_socket, msg.m_size, msg.m_data) {}
        
        // move const
        Message(Message&& msg) {
            m_data = msg.m_data;
            m_size = msg.m_size;
            m_socket = msg.m_socket;
            
            msg.m_data = nullptr;
        }
        
        ~Message() {
            if(m_data) {
                delete[] m_data;
            }
        }
        
        auto dispatch() -> void {
            send(m_socket, m_data, m_size, MSG_NOSIGNAL);
        }
        
        auto socket() -> int {
            return m_socket;
        }
        
        auto size() -> size_t {
            return m_size;
        }
        
        auto data() -> char* {
            return m_data;
        }
        
    private:
        char* m_data;
        size_t m_size;
        int m_socket;
};
