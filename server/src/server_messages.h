#pragma once

#include "app.h"
#include <stdint.h>

typedef enum ServerMessage {
    MSG_INVALID,
    MSG_CREATE_GAME,
    MSG_JOIN_GAME,
    MSG_PUT_STONE,
    MSG_MOVE_STONE,
    MSG_CONNECT,
    MSG_DISCONNECT,
    SERVER_MESSAGES_COUNT,
} ServerMessage;

typedef enum ClientMessage {
    OK,
    FAIL,
    GAME_CREATED,
    GAME_JOINED,
    GAME_OPPONENT_JOINED,
    PING,
} ClientMessage;

typedef struct __attribute__((packed)) RawMessage {
    uint32_t msg;
    uint64_t arg1;
    uint64_t arg2;
    uint32_t checksum;
} RawMessage;

typedef struct Message {
    ServerMessage msg;
    uint64_t arg1;
    uint64_t arg2;
    uint32_t checksum;
} Message;

typedef struct RequestData {
    Application* app;
    int client_socket;
} RequestData;

char* get_msg_str(ServerMessage msg);
void* serve_request(void* arg);
