#include "receiver.hpp"
#include "server.hpp"
#include <chrono>
#include <iostream>

void Receiver::push_message(Message msg) {
    m_queue.push(std::move(msg));
}

void Receiver::run(Receiver* receiver) {
    while(true) {
        while(!receiver->m_queue.empty()) {
            Message msg = receiver->m_queue.pop();
            std::cout << "popping message";
            
            receiver->m_server->sockets().atomic_op<void>([&msg] (std::unordered_map<int, Socket>& map) {
                Socket& sock = map.at(msg.socket());
                sock.machine()->handle_message(msg);
            });
        }
        
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }
}
