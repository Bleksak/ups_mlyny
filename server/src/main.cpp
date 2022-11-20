#include "server/server.hpp"

int main() {
    Server server(2000);
    server.start().join();
    
    return 0;
}
