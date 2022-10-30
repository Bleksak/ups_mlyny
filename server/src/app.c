#include "app.h"
#include <limits.h>
#include <stdint.h>

Application* app_new(int server_socket) {
    Application* app = calloc(1, sizeof(Application));
    
    app->server_socket = server_socket;
    app->player_identifier = 1;
    
    return app;
}
