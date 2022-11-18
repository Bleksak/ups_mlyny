#include "sender.hpp"

void Sender::push_message(Message msg) {
    m_queue.push(msg);
}

void Sender::run(Sender& sender) {
    while(true) {
        while(!sender.m_queue.empty()) {
            Message msg = sender.m_queue.pop();
            // TODO: send the msg
        }
    }
}
