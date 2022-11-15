#include "game_destroyer.hpp"
#include "server.hpp"
#include <chrono>
#include <thread>

void GameDestroyer::run(GameDestroyer* destroyer) {
    while(true) {
        destroyer->m_server.games().atomic_op<void>([](std::vector<Game>& games) {
            auto current_time = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch());
            auto it = games.begin();
            
            while(it != games.end()) {
                auto timer_opt = it->timer();
                if(timer_opt.has_value()) {
                    auto timer = timer_opt.value();
                    auto diff = std::chrono::duration_cast<std::chrono::milliseconds>( current_time - timer );
                    if(diff.count() >= TIMEOUT) {
                        it = games.erase(it);
                        continue;
                    }
                }
                
                it++;
            }
        });
        
        // sleep for a longer time, we dont rush into removing games :D
        // (mostly because it locks the vector mutex for O(n) time)
        std::this_thread::sleep_for(std::chrono::seconds(1));
        
    }
}
