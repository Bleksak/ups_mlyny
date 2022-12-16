use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, Weak};
use std::thread::spawn;
use std::time::Instant;

use super::client::Client;
use super::message::Message;
use crate::game::{player::Player, Game};
use crate::machine::Machine;

pub struct MessageReceiver {
    games: Arc<Mutex<Vec<Arc<Game>>>>,
    players: Mutex<HashMap<i32, Weak<Player>>>,
    disconnect_channel: Sender<Weak<Client>>,
    last_deletion_cycle: Instant,
}

impl MessageReceiver {
    pub fn new(disconnect_channel: Sender<Weak<Client>>) -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            players: Mutex::new(HashMap::new()),
            disconnect_channel,
            last_deletion_cycle: Instant::now(),
        }
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

    pub fn deletion_cycle_proc(games: Arc<Mutex<Vec<Arc<Game>>>>) {
        let mut indices = vec![];
        let mut games = games.lock().unwrap();
        for (index, game) in games.iter().enumerate() {
            if game.can_delete() {
                indices.push(index);
            }
        }

        for index in indices.iter().rev() {
            games.remove(*index);
        }
    }

    pub fn disconnect(&self, client: Weak<Client>) {
        self.disconnect_channel.send(client).unwrap();
    }

    pub fn run(&mut self, channel: Receiver<(Arc<Client>, Message)>) {
        loop {
            if self.last_deletion_cycle.elapsed().as_secs() >= 5 {
                let games = self.games.clone();
                spawn(|| Self::deletion_cycle_proc(games));
                self.last_deletion_cycle = Instant::now();
            }

            while let Ok((client, msg)) = channel.try_recv() {
                match msg {
                    Message::Ping => {
                        if let Ok(_) = client.write(Message::Pong.serialize().as_slice()) {
                            // println!("sent pong!");
                        }
                    }
                    Message::Disconnect => {
                        if let Some(player) = self.find_player(client.sock_fd()) {
                            if let Some(game) = player.game().upgrade() {
                                game.notify_disconnect(client);
                            }
                        }
                    }

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
