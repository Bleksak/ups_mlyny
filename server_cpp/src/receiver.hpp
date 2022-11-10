#pragma once

#include <chrono>
#include <iostream>
#include <thread>
#include "cqueue.hpp"
#include "message.hpp"
#include <sys/socket.h>
#include <functional>
#include <unordered_map>
#include "command.hpp"

class Server;

class Receiver {
    public:
        Receiver(Server& server) : m_server(server) {
            std::thread(Receiver::run, this).detach();
        }
        
        auto push_message(RecvMessage msg) -> void {
            m_queue.push(msg);
        }
        
    private:
        [[noreturn]]
        static auto run(Receiver* receiver) -> volatile void {
            while(true) {
                while(!receiver->m_queue.empty()) {
                    std::cout << "waat" << std::endl;
                    RecvMessage msg = receiver->m_queue.pop();
                    std::cout << msg.socket() << std::endl;
                    
                    auto fn_it = ctor_map.find(msg.type());
                    if(fn_it == ctor_map.end()) {
                        continue;
                    }
                    
                    fn_it->second(receiver->m_server, std::move(msg));
                }
                
                std::this_thread::sleep_for(std::chrono::milliseconds(20));
            }
        }
        
        ConcurrentQueue<RecvMessage> m_queue;
        Server& m_server;
        
        inline static std::unordered_map<MessageType, std::function<void (Server&, RecvMessage)>> ctor_map {
            // std::make_pair(MessageType::OK,),
            // std::make_pair(MessageType::NOK,,
            // std::make_pair(MessageType::PLAYER_INIT, "TELL ME WHO YOU ARE\n"),
            std::make_pair(MessageType::PLAYER_INIT_RES, Command::player_init),
            // std::make_pair(MessageType::PLAYER_PUT, "\n"),
            // std::make_pair(MessageType::PLAYER_MV, "IM GONNA DO WHATS CALLED A PRO-GAMER MOVE\n"),
            // std::make_pair(MessageType::PLAYER_TAKE, "NIGGAS GONNA ROB\n"),
            std::make_pair(MessageType::PING, Command::ping),
            // std::make_pair(MessageType::PONG, "WHOS THERE?\n"),
        };
};
