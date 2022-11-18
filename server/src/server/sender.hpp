#pragma once

#include "message.hpp"
#include "../container/cqueue.hpp"
#include <thread>

class Server;
class Sender {
    public:
        Sender() {
            std::thread(Sender::run, *this).detach();
        }
        
        void push_message(Message msg);
        
    private:
        static void run(Sender& sender);
        
        ConcurrentQueue<Message> m_queue;
};