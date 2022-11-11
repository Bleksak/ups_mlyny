#include "command.hpp"
#include "server.hpp"
#include <iostream>

void Command::player_init(Server& server, RecvMessage data) {
    
}

void Command::ping(Server& server, RecvMessage data) {
    std::cout << "PONG!" << std::endl;
    server.sender().push_message(Message(data.socket(), MessageType::PONG, 0, nullptr));
}
