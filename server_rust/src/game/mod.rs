use crate::server::client::Client;

use self::player::Player;
use std::sync::{Mutex, Arc};

pub mod player;

pub struct Game {
    players: [Player; 2],
}

impl<'recv> Game {
    pub fn new() -> Self {
        Self {
            players: [Player::new(), Player::new()]
        }
    }
    
    pub fn connect(&mut self, username: String, client: Arc<Mutex<Client>>) -> bool {
        
        // 1. check if username is connected to the game
        
        // Arc<Mutex<Game>>
        
        for player in self.players.iter_mut() {
            if let Some(name) = player.name() {
                if name == &username {
                    if player.client().upgrade().is_none() {
                        player.bind(Arc::downgrade(&client));
                        return true;
                    }
                }
            }
        }
        
        // 2. find first empty and join
        for player in self.players.iter_mut() {
            if player.name().is_none() {
                player.set_name(username);
                player.bind(Arc::downgrade(&client));
                return true;
            }
        }
        
        false
    }
}