pub mod message;
pub mod receiver;
pub mod client;


use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};

use crate::machine::Machine;
use crate::server::message::Message;

use self::client::Client;
use self::receiver::MessageReceiver;

pub struct Server {
    socket: TcpListener,
    receiver: Arc<Mutex<MessageReceiver>>,
    clients: Vec<Arc<Mutex<Client>>>,
    client_channel: (Sender<Client>, Receiver<Client>),
    receiver_sender: Sender<(Arc<Mutex<Client>>, Message)>,
}

impl Server {
    pub fn new(port: u16, receiver: MessageReceiver) -> Result<Self, io::Error> {
        if port == 0 {
            Err(io::Error::new(io::ErrorKind::Other, "Port 0 is not allowed!"))
        } else {
            let sender = receiver.sender();
            
            Ok(Self {
                socket: TcpListener::bind(format!("127.0.0.1:{}", port))?,
                receiver: Arc::new(Mutex::new(receiver)),
                clients: vec![],
                client_channel: mpsc::channel(),
                receiver_sender: sender,
            })
        }
    }
    
    fn process_request(&self, client: &mut Arc<Mutex<Client>>, data: &Vec<u8>) {
        // TODO: maybe we need to parse multiple messages first
        if let Some(message) = Message::deserialize(data) {
            self.receiver_sender.send( (client.clone(), message) ).unwrap();
        }
    }
    
    fn run(mut self) {
        loop {
            if let Ok(client) = self.client_channel.1.try_recv() {
                let client = Arc::new(Mutex::new(client));
                println!("Got new client!");
                client.lock().unwrap().set_nonblocking(true).expect("Failed to set client nonblocking");
                client.lock().unwrap().set_machine(Machine::new(Arc::downgrade(&client), self.receiver.clone()));
                self.clients.push(client);
            }
            
            let mut disconnect = vec![];
            
            let mut data_vec  = Vec::new();
            for (index, client) in self.clients.iter_mut().enumerate() {
                if let Some(data) = client.lock().unwrap().read_all() {
                    if data.len() == 0 {
                        disconnect.push(index);
                    } else {
                        data_vec.push((client.clone(), data));
                    }
                }
            }
            
            for (mut client, data) in data_vec {
                self.process_request(&mut client, &data);
            }
            
            if disconnect.len() == 1 {
                self.clients.swap_remove(disconnect[0]);
            } else {
                for delete in disconnect.iter().rev() {
                    println!("disconnecting!");
                    self.clients.remove(*delete);
                }
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    pub fn start(self) {
        self.socket.set_nonblocking(true).unwrap();
        
        let socket = self.socket.try_clone().unwrap();
        let tx = self.client_channel.0.clone();
        
        let recv = self.receiver.clone();
        
        let receiver = std::thread::spawn(move|| {
            MessageReceiver::run(recv);
        });
        
        let acceptor = std::thread::spawn(move || {
            for client in socket.incoming() {
                if let Ok(client) = client {
                    tx.send(Client::new(client)).unwrap();
                }
            }
        });
        
        let thread = std::thread::spawn(move|| {
            self.run();
        });
        
        receiver.join().unwrap();
        thread.join().unwrap();
        acceptor.join().unwrap();
    }
}
