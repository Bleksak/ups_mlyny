use std::sync::mpsc::Receiver;
use std::sync::{Arc, Weak, Mutex};
use std::collections::HashMap;

use super::client::Client;
use super::message::Message;
use crate::game::{Game, player::Player};

// TODO: all of this is SINGLE THREADED
// WE DONT SPAWN ANY MORE THREADS
// SOO.. JUST FUCKING DONT USE ARC AND MUTEX

pub struct MessageReceiver {
    games: Mutex<Vec<Arc<Game>>>,
    players: HashMap<u32, Weak<Player>>,
}

impl MessageReceiver {
    pub fn new() -> Self {
        Self { games: Mutex::new(vec![]), players: HashMap::new() }
    }
    
    pub fn run(self, channel: Receiver<(Arc<Client>, Message)>) {
        loop {
            while let Ok((client, msg)) = channel.try_recv() {
                match msg {
                    Message::PING => {
                        if let Ok(_) = client.write(Message::PONG.serialize().as_slice()) {
                            // println!("sent pong!");
                        }
                    },
                    Message::PONG => {
                        // TODO: refresh player's timer
                    },
                    
                    _ => {
                        client.machine().lock().unwrap().handle_message(msg, &self, Arc::downgrade(&client));
                    }
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    pub fn games(&self) -> &Mutex<Vec<Arc<Game>>> {
        &self.games
    }
    
}
