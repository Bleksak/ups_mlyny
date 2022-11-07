#pragma once

#include <chrono>
#include <thread>
#include "cqueue.hpp"
#include "message.hpp"
#include <sys/socket.h>

class Receiver {
    public:
        Receiver() {
            std::thread(Receiver::run, this).detach();
        }
        
        auto push_message(Message msg) -> void {
            m_queue.push(std::move(msg));
        }
        
    private:
        
        [[noreturn]]
        static auto run(Receiver* receiver) -> void {
            while(true) {
                while(!receiver->m_queue.empty()) {
                    Message msg = receiver->m_queue.pop();
                    
                    
                }
                
                std::this_thread::sleep_for(std::chrono::milliseconds(20));
            }
        }
        
        ConcurrentQueue<Message> m_queue;
};
