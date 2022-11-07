#pragma once

#include <cstring>
#include <iostream>
#include <sstream>
#include <unordered_map>
#include <vector>
#include <sys/socket.h>

#define JEBANY_WROT 1
#if JEBANY_WROT == 1
    #define ADD_SIZE(t) (t.length() * sizeof(char))
#else
    #define ADD_SIZE(t) (sizeof(uint32_t))
#endif

enum MessageType {
    OK,
    NOK,
    PLAYER_INIT,
    PLAYER_INIT_RES,
    PING,
    PONG,
};



class Message {
    public:
        Message(int socket, MessageType type, size_t size, char* data) : m_size(size), m_type(type), m_socket(socket) {
            // to nefunguje jak si myslis blbecku
            // asi je lepsi udelat std vector
            // 
            std::ostringstream oss(m_data);
            
            uint32_t final_msg_size = size + sizeof(uint32_t);
            
            const std::string& str = msg_type_str(type);
            #if JEBANY_WROT == 1
                final_msg_size += str.size();
                oss.write(reinterpret_cast<char*>(std::addressof(final_msg_size)), sizeof(uint32_t));
                oss << str;
            #else
                oss.write( reinterpret_cast<char*>(std::addressof(final_msg_size)), sizeof(uint32_t));
            #endif
            
            if(size && data != nullptr) {
                oss << std::string(data, size);
            }
            
            oss.flush();
            
            std::cout << m_data;
        }
        
        // copy ctor
        // Message(const Message& msg) : Message(msg.m_socket, msg.m_type, msg.m_size, msg.m_data) {
        //     m_type = msg.m_type;
        //     m_socket = msg.m_socket;
        //     m_size = msg.m_size;
        //     m_data = new char[m_size + ADD_SIZE(msg_type_str(m_type))];
        //     std::memcpy(m_data, msg.m_data, m_size + ADD_SIZE(msg_type_str(m_type)));
        // }
        
        // move ctor
        Message(Message&& msg) {
            m_type = msg.m_type;
            m_socket = msg.m_socket;
            m_size = msg.m_size;
            m_data = std::move(msg.m_data);
        }
        
        auto dispatch() -> void {
            send(m_socket, data(), m_data.size(), MSG_NOSIGNAL);
        }
        
        auto msg_type_str(MessageType type) -> const std::string& {
            return msg_type_strs.at(type);
        }
        
        auto socket() -> int {
            return m_socket;
        }
        
        auto size() -> size_t {
            return m_size;
        }
        
        auto data() -> const char* {
            return m_data.c_str();
        }
        
    private:
        std::string m_data;
        size_t m_size;
        MessageType m_type;
        int m_socket;
        
        inline const static std::unordered_map<MessageType, std::string> msg_type_strs = {
            std::make_pair(MessageType::OK, "LIFE IS GOOD!"),
            std::make_pair(MessageType::NOK, "LIFE IS BAD!"),
            std::make_pair(MessageType::PLAYER_INIT, "TELL ME WHO YOU ARE!"),
            std::make_pair(MessageType::PLAYER_INIT_RES, "I AM TELLING YOU WHO I AM!"),
            std::make_pair(MessageType::PING, "KNOCK KNOCK!"),
            std::make_pair(MessageType::PONG, "WHOS THERE?!"),
        };
};
