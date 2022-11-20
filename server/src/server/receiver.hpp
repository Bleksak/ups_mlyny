#pragma once

#include "../container/cqueue.hpp"
#include "message.hpp"

#include <thread>

class Server;
class Receiver {
    public:
        Receiver(Server* server) : m_server(server) {
            std::thread(Receiver::run, this).detach();
        }
        
        void push_message(Message msg);
        
    private:
        static void run(Receiver* receiver);
        
        Server* m_server;
        ConcurrentQueue<Message> m_queue;
};
