use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Weak};

use super::client::Client;
use super::message::Message;
use crate::game::{player::Player, Game};
use crate::machine::Machine;


pub struct MessageReceiver {
    games: Mutex<Vec<Arc<Game>>>,
    players: Mutex<HashMap<i32, Weak<Player>>>,
}

impl MessageReceiver {
    pub fn new() -> Self {
        Self { games: Mutex::new(vec![]), players: Mutex::new(HashMap::new()) }
    }
    
    fn find_player(&self, fd: i32) -> Option<Arc<Player>> {
        let mut lock = self.players.lock().unwrap();
        if let Some(player) = lock.get(&fd).and_then(|p| p.upgrade()) {
            if player.client().upgrade().is_none() {
                lock.remove(&fd);
                return None;
            } else {
                return Some(player);
            }
        }
        
        None
    }
    
    pub fn run(&self, channel: Receiver<(Arc<Client>, Message)>) {
        loop {
            while let Ok((client, msg)) = channel.try_recv() {
                match msg {
                    Message::Ping => {
                        if let Ok(_) = client.write(Message::Pong.serialize().as_slice()) {
                            // println!("sent pong!");
                        }
                    },
                    Message::Disconnect => {
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            if let Some(game ) = player.game().upgrade() {
                                game.notify_disconnect(client);
                            }
                        } else {
                            // Machine::new_client_init(msg, &self, Arc::downgrade(&client));
                        }
                    },
                    
                    _ => {
                        println!("got message: {:?}", msg);
                        // 1. try to find player for given client
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            let machine = player.machine().clone();
                            machine.handle_message(msg, &self, player);
                        } else {
                            Machine::new_client_init(msg, &self, Arc::downgrade(&client));
                        }
                    }
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(40));
        }
    }
    
    pub fn games(&self) -> &Mutex<Vec<Arc<Game>>> {
        &self.games
    }
    
    pub fn players(&self) -> &Mutex<HashMap<i32, Weak<Player>>> {
        &self.players
    }
    
}
