#include "receiver.hpp"
#include <chrono>

void Receiver::push_message(Message msg) {
    m_queue.push(std::move(msg));
}

void Receiver::run(Receiver& receiver) {
    while(true) {
        while(!receiver.m_queue.empty()) {
            Message msg = receiver.m_queue.pop();
        }
                
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }
}
