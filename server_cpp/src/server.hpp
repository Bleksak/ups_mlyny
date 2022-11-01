#pragma once

#include <thread>

class Server {
    public:
        Server(std::uint16_t port);
        std::thread& listen();
    private: 
        const int m_socket;
};
