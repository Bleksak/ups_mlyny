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
    INVALID,
    OK,
    NOK,
    PLAYER_INIT,
    PLAYER_INIT_CREATE,
    PLAYER_INIT_JOIN,
    PLAYER_PUT,
    PLAYER_MV,
    PLAYER_TAKE,
    PING,
    PONG,
    PLAYER_INIT_USERNAME_INVALID,
    PLAYER_INIT_USERNAME_USED
};

class Message {
    public:
        Message(int socket, MessageType type, size_t size, char* data) : m_size(size), m_type(type), m_socket(socket) {
            uint32_t final_msg_size = size + sizeof(uint32_t);
            
            const std::string& str = msg_type_str(type);
            #if JEBANY_WROT == 1
                final_msg_size += str.size();
            #endif
            
            char* c_ptr = reinterpret_cast<char*>(std::addressof(final_msg_size));
            m_data.insert(m_data.end(), c_ptr[0]);
            m_data.insert(m_data.end(), c_ptr[1]);
            m_data.insert(m_data.end(), c_ptr[2]);
            m_data.insert(m_data.end(), c_ptr[3]);
            
            #if JEBANY_WROT == 1
                m_data.insert(m_data.end(), str.begin(), str.end());
            #endif
            
            if(size && data != nullptr) {
                std::string data_string(data, size);
                std::cout << data_string;
                m_data.insert(m_data.end(), data_string.begin(), data_string.end());
            }
            
            std::string mesg(m_data.begin(), m_data.end());
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
            return m_data.data();
        }
        
        inline const static std::unordered_map<MessageType, std::string> msg_type_strs = {
            std::make_pair(MessageType::OK, "LIFE IS GOOD\n"),
            std::make_pair(MessageType::NOK, "LIFE IS BAD\n"),
            std::make_pair(MessageType::PLAYER_INIT, "TELL ME WHO YOU ARE\n"),
            std::make_pair(MessageType::PLAYER_INIT_CREATE, "BY YOUR HAND ALL THINGS WERE MADE... EVEN ME\n"),
            std::make_pair(MessageType::PLAYER_INIT_JOIN, "I AM TELLING YOU WHO I AM\n"),
            std::make_pair(MessageType::PLAYER_PUT, "SIT DOWN\n"),
            std::make_pair(MessageType::PLAYER_MV, "IM GONNA DO WHATS CALLED A PRO-GAMER MOVE\n"),
            std::make_pair(MessageType::PLAYER_TAKE, "NIGGAS GONNA ROB\n"),
            std::make_pair(MessageType::PING, "KNOCK KNOCK\n"),
            std::make_pair(MessageType::PONG, "WHOS THERE?\n"),
            std::make_pair(MessageType::PLAYER_INIT_USERNAME_INVALID, "YOUR ARGUMENT IS INVALID\n"),
            std::make_pair(MessageType::PLAYER_INIT_USERNAME_USED, "IF I LICK IT ITS MINE\n"),
        };
        
    private:
        std::vector<char> m_data;
        size_t m_size;
        MessageType m_type;
        int m_socket;
        
};

class RecvMessage {
    public:
        RecvMessage(int socket, MessageType type, std::vector<char> data) : m_socket(socket), m_type(type), m_data(data) {}
        
        auto socket() const -> int {
            return m_socket;
        }
        
        auto type() const -> MessageType {
            return m_type;
        }
        
        auto data() const -> const std::vector<char>& {
            return m_data;
        }
        
        static auto get_type(std::string& str) -> MessageType {
            for(auto& item : Message::msg_type_strs) {
                if(item.second == str) {
                    return item.first;
                }
            }
            
            return MessageType::INVALID;
        }
    private:
        int m_socket;
        MessageType m_type;
        std::vector<char> m_data;
};
