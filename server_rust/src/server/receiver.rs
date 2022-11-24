use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Weak, Mutex};
use std::collections::HashMap;

use super::client::Client;
use super::message::Message;
use crate::game::{Game, player::Player};

pub struct MessageReceiver {
    channel: (Sender<(Arc<Mutex<Client>>, Message)>, Receiver<(Arc<Mutex<Client>>, Message)>),
    games: Arc<Mutex<Vec<Arc<Mutex<Game>>>>>,
    players: HashMap<u32, Weak<Player>>,
}

impl MessageReceiver {
    pub fn new(channel: (Sender<(Arc<Mutex<Client>>, Message)>, Receiver<(Arc<Mutex<Client>>, Message)>)) -> Self {
        Self { channel, games: Arc::new(Mutex::new(vec![])), players: HashMap::new() }
    }
    
    pub fn sender(&self) -> Sender<(Arc<Mutex<Client>>, Message)> {
        self.channel.0.clone()
    }
    
    pub fn run(receiver: Arc<Mutex<Self>>) {
        loop {
            while let Ok((client, msg)) = receiver.lock().unwrap().channel.1.try_recv() {
                println!("got message");
                
                match msg {
                    Message::PING => {
                        if let Ok(_) = client.lock().unwrap().write(Message::PONG.serialize().as_slice()) {
                            // println!("sent pong!");
                        }
                    },
                    Message::PONG => {
                        // TODO: refresh player's timer
                    },
                    
                    _ => {
                        std::thread::spawn(move|| {
                            let machine = client.lock().unwrap().machine();
                            machine.lock().unwrap().handle_message(msg);
                        });
                    }
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    pub fn games(&self) -> Arc<Mutex<Vec<Arc<Mutex<Game>>>>> {
        self.games.clone()
    }
    
}
