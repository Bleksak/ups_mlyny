#include "server.hpp"

auto main(int argc, const char* argv[]) -> int {
    Server server(2000);
    std::thread thread = server.start();
    thread.join();
    return 0;
}