#pragma once

#include <chrono>
#include <thread>
#include "cqueue.hpp"
#include "message.hpp"
#include <sys/socket.h>

class Sender {
    public:
        Sender() {
            std::thread(Sender::run, this).detach();
        }
        
        auto push_message(Message msg) -> void {
            m_queue.push(std::move(msg));
        }
        
    private:
        
        [[noreturn]]
        static auto run(Sender* sender) -> void {
            while(true) {
                while(!sender->m_queue.empty()) {
                    sender->m_queue.pop().dispatch();
                }
                
                std::this_thread::sleep_for(std::chrono::milliseconds(20));
            }
        }
        
        ConcurrentQueue<Message> m_queue;
};
