pub mod message;
pub mod receiver;
pub mod client;

use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};

use crate::server::message::Message;

use self::client::Client;

pub struct Server {
    socket: TcpListener,
    clients: Vec<Client>,
    client_channel: (Sender<Client>, Receiver<Client>),
    receiver_sender: Sender<(Client, Message)>,
}

impl Server {
    pub fn new(port: u16, recv_sender: Sender<(Client, Message)>) -> Result<Self, io::Error> {
        if port == 0 {
            Err(io::Error::new(io::ErrorKind::Other, "Port 0 is not allowed!"))
        } else {
            Ok(Self {
                socket: TcpListener::bind(format!("127.0.0.1:{}", port))?,
                clients: vec![],
                client_channel: mpsc::channel(),
                receiver_sender: recv_sender,
            })
        }
    }
    
    fn read_all(client: &mut TcpStream) -> Option<Vec<u8>> {
        let mut data : Vec<u8> = Vec::with_capacity(512);
        data.resize(512, 0);
        
        let mut read_bytes = 0;
        
        let read = client.read(&mut data[..]);
        
        if let Err(err) = read {
            match err.kind() {
                io::ErrorKind::WouldBlock => {
                    return None;
                },
                
                _ => {}
            }
        } else {
            read_bytes += read.unwrap();
        }
        
        loop {
            data.resize(read_bytes + 512, 0);
            let read = client.read(&mut data[read_bytes..read_bytes+512]);
            
            if let Err(err) = read {
                match err.kind() {
                    io::ErrorKind::WouldBlock => {
                        data.truncate(read_bytes);
                        return Some(data)
                    },
                
                    _ => {
                        return None;
                    }
                }
            } else {
                let read = read.unwrap();
                if read == 0 {
                    data.truncate(read_bytes);
                    return Some(data);
                }
                read_bytes += read;
            }
        }
    }
    
    fn process_request(&self, client: &mut Client, data: &Vec<u8>) {
        // TODO: maybe we need to parse multiple messages first
        
        if let Some(message) = Message::deserialize(data) {
            self.receiver_sender.send( (client.clone(), message) ).unwrap();
        }
    }
    
    fn run(mut self) {
        loop {
            if let Ok(client) = self.client_channel.1.try_recv() {
                println!("Got new client!");
                client.stream().set_nonblocking(true).expect("Failed to set client nonblocking");
                self.clients.push(client);
            }
            
            let mut disconnect = vec![];
            
            let mut data_vec: Vec<(Client, Vec<u8>)> = Vec::new();
            for (index, client) in self.clients.iter_mut().enumerate() {
                if let Some(data) = Self::read_all(client.stream_mut()) {
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
        
        let acceptor = std::thread::spawn(move || {
            for client in socket.incoming() {
                if let Ok(client) = client {
                    tx.send(Client::new(client)).unwrap();
                }
            }
        });
        
        let thread = std::thread::spawn(|| {
            self.run();
        });
        
        thread.join().unwrap();
        acceptor.join().unwrap();
    }
}
