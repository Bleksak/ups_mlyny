#include "sender.hpp"
#include <sys/socket.h>

void Sender::push_message(Message msg) {
    m_queue.push(std::move(msg));
}

void Sender::run(Sender* sender) {
    while(true) {
        while(!sender->m_queue.empty()) {
            Message msg = sender->m_queue.pop();
            // TODO: send the msg
            send(msg.socket(), msg.data().data(), msg.data().size(), 0);
        }
    }
}
