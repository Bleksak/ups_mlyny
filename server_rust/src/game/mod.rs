use crate::server::{client::Client, message::Message};

use self::player::Player;
use std::{sync::{Mutex, Weak}, io::Write};

pub mod player;

pub struct Game {
    players: [Player; 2],
}

impl<'recv> Game {
    pub fn new() -> Self {
        println!("Creating a game");
        Self {
            players: [Player::new(), Player::new()]
        }
    }
    
    fn notify_join(&mut self) {
        for player in self.players.iter_mut() {
            if let Some(client) = player.client().upgrade() {
                if let Ok(_) = client.lock().unwrap().write_all(&Message::PLAYER_JOINED.serialize()) {
                    println!("notified player!");
                }
            }
        }
    }
    
    pub fn connect(&mut self, username: &String, client: Weak<Mutex<Client>>) -> bool {
        
        // 1. check if username is connected to the game
        println!("Trying to connect {}", username);
        
        for player in self.players.iter_mut() {
            if let Some(name) = player.name() {
                if name == username {
                    if player.client().upgrade().is_none() {
                        println!("Connecting {}", username);
                        player.bind(client.clone());
                        self.notify_join();
                        return true;
                    }
                }
            }
        }
        
        // 2. find first empty and join
        for player in self.players.iter_mut() {
            if player.name().is_none() {
                println!("Connecting {}", username);
                player.set_name(username.clone());
                player.bind(client.clone());
                self.notify_join();
                return true;
            }
        }
        
        false
    }
    
    pub fn player(&self, username: &str) -> Option<&Player> {
        println!("searching for player {}", username);
        self.players.iter().find(|player| {
            if let Some(playername) = player.name() {
                return playername == username;
            }
            
            false
        })
    }
    
    pub fn player_mut(&mut self, username: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|player| {
            if let Some(playername) = player.name() {
                return playername == username;
            }
            
            false
        })
    }
}