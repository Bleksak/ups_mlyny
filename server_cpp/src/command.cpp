#include "command.hpp"
#include "message.hpp"
#include "server.hpp"
#include <iostream>
#include "validator.hpp"
#include "game.hpp"

void Command::player_init_create(Server& server, RecvMessage data) {
    std::string username(data.data().begin(), data.data().end());
    if(!Validator::username(username)) {
        // SEND PACKET THAT USERNAME IS NOT OK
        server.sender().push_message(Message(data.socket(), MessageType::PLAYER_INIT_USERNAME_INVALID, 0, nullptr));
        return;
    }
    
    // 1. find socket by name
    Socket* existing_username = server.sockets().find([&username](const Socket& sock) {
        return username == sock.identifier();
    });
    
    Socket* existing_id = server.sockets().find([&data](const Socket& sock){
        return data.socket() == sock.socket();
    });
    
    
    // when do we call Socket(data.socket(), username) ?
    // ALWAYS!
    // but we need some checks:
    
    // if username exists:
    //      if id exists:
    //          can_create = !(username_in_game || id_in_game)
    //      else
    //          can_create = !(username_in_game)
    // else:
    //      if id exists:
    //          can_create = !(id_in_game)
    //      else
    //          can_create = TRUE
    
    // A => id exists
    // B => username exists
    // C => id in game
    // D => username in game
    // X => can create
    // . => OK
    // U => USERNAME TAKEN
    // I => YOU ARE IN GAME
   
    //    A B C D X M
    //    0 0 0 0 1 .
    //    0 1 0 0 1 .
    //    0 1 0 1 0 U
    //    1 0 0 0 1 .
    //    1 0 1 0 0 I
    //    1 1 0 0 1 .
    //    1 1 0 1 0 U
    //    1 1 1 0 0 I
    //    1 1 1 1 0 I
    
    bool id_exists = existing_id != nullptr;
    bool username_exists = existing_username != nullptr;
    
    Game* game_id = id_exists ? server.games().find([&existing_id](const Game& game) {
        return game.players()[0].name() == existing_id->identifier() || game.players()[1].name() == existing_id->identifier();
    }) : nullptr;
    
    bool game_id_exists = game_id != nullptr;
    bool game_id_connected = false;
    
    if(game_id) {
        game_id_connected = (game_id->players()[0].name() == existing_id->identifier() && game_id->connected()[0]) || (game_id->players()[1].name() == existing_id->identifier() && game_id->connected()[1]);
    }
    
    Game* game_username = username_exists ? server.games().find([&existing_username](const Game& game) {
        return game.players()[0].name() == existing_username->identifier() || game.players()[1].name() == existing_username->identifier();
    }) : nullptr;
    
    
    bool game_username_exists = game_username != nullptr;
    bool game_username_connected = false;
    
    if(game_username) {
        game_username_connected = (game_username->players()[0].name() == existing_username->identifier() && game_username->connected()[0]) || (game_username->players()[1].name() == existing_username->identifier() && game_username->connected()[1]);
    }
    
    bool game_create = !(game_username_connected || game_id_connected);
    bool destroy_id_game = (game_id_exists && game_create);
    bool destroy_username_game = (game_username_exists && game_create);
    
    if(destroy_id_game) {
        server.games().find_and_erase([&existing_id](const Game& game) {
            return game.players()[0].name() == existing_id->identifier() || game.players()[1].name() == existing_id->identifier();
        });
    }
    
    if(destroy_username_game) {
        server.games().find_and_erase([&existing_username](const Game& game) {
            return game.players()[0].name() == existing_username->identifier() || game.players()[1].name() == existing_username->identifier();
        });
    }
    
    if(game_create) {
        server.sockets().find_and_erase([&data](const Socket& sock) {
            return sock.socket() == data.socket();
        });
        
        server.sockets().find_and_erase([&username](const Socket& sock) {
            return sock.identifier() == username;
        });
        
        Socket new_socket(data.socket(), username);
        
        server.sockets().insert_sorted(std::move(new_socket), [](const Socket& sock1, const Socket& sock2) {
            return sock1.socket() > sock2.socket();
        });
        
        server.games().push_back(Game(Player(std::move(username))));
        server.sender().push_message(Message(data.socket(), MessageType::OK, 0, nullptr));
    } else {
        if(game_id_connected) {
            // you are connected
            server.sender().push_message(Message(data.socket(), MessageType::OK, 0, nullptr));
        } else if(game_username_connected) {
            // username occupied
            server.sender().push_message(Message(data.socket(), MessageType::PLAYER_INIT_USERNAME_USED, 0, nullptr));
        }
    }
}

void Command::ping(Server& server, RecvMessage data) {
    std::cout << "PONG!" << std::endl;
    server.sender().push_message(Message(data.socket(), MessageType::PONG, 0, nullptr));
}
