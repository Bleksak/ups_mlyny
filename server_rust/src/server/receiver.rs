use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Weak, Mutex};
use std::collections::HashMap;

use super::client::Client;
use super::message::Message;
use crate::game::{Game, player::Player};

pub struct MessageReceiver {
    channel: (Sender<(Arc<Mutex<Client>>, Message)>, Receiver<(Arc<Mutex<Client>>, Message)>),
    games: Vec<Arc<Mutex<Game>>>,
    players: HashMap<u32, Weak<Player>>,
}

impl MessageReceiver {
    pub fn new(channel: (Sender<(Arc<Mutex<Client>>, Message)>, Receiver<(Arc<Mutex<Client>>, Message)>)) -> Self {
        Self { channel, games: vec![], players: HashMap::new() }
    }
    
    pub fn sender(&self) -> Sender<(Arc<Mutex<Client>>, Message)> {
        self.channel.0.clone()
    }
    
    pub fn run(receiver: Arc<Mutex<Self>>) {
        loop {
            while let Ok((client, msg)) = receiver.lock().unwrap().channel.1.try_recv() {
                match msg {
                    Message::PING => {
                        if let Ok(_) = client.lock().unwrap().write(Message::PONG.serialize().as_slice()) {
                            println!("sent pong!");
                        }
                    },
                    Message::PONG => {
                        // TODO: refresh player's timer
                    },
                    
                    _ => {
                        std::thread::spawn(move|| {
                            client.lock().unwrap().machine_mut().handle_message(msg);
                        });
                    }
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    // TODO: check if user is in game.. if user is disconnected
    pub fn create_game(&mut self, username: &str, client: Weak<Mutex<Client>>) {
        // client cannot be in game
        // but username can
        for game in self.games.iter() {
            
        }
    }
    
}
