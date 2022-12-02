pub mod message;
pub mod receiver;
pub mod client;

use std::collections::BinaryHeap;
use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Weak};
use std::sync::mpsc::{self, Receiver, Sender};

use crate::server::message::Message;

use self::client::Client;
use self::receiver::MessageReceiver;

pub struct Server {
    socket: TcpListener,
    clients: Vec<Arc<Client>>,
    client_channel: (Sender<Client>, Receiver<Client>),
    recv_channel: Sender<(Arc<Client>, Message)>,
    disconnect_channel: Receiver<Weak<Client>>
}

impl Server {
    fn process_request(&self, client: Arc<Client>, data: &Vec<u8>) {
        if let Some(message) = Message::deserialize(data) {
            println!("{:?}", message);
            self.recv_channel.send( (client.clone(), message) ).unwrap();
        }
    }
    
    fn run(mut self) {
        loop {
            if let Ok(client) = self.client_channel.1.try_recv() {
                println!("Got new client!");
                let client = Arc::new(client);
                client.lock().unwrap().set_nonblocking(true).expect("Failed to set client nonblocking");
                self.clients.push(client);
            }
            
            let mut disconnect = BinaryHeap::new();
            
            if let Ok(client) = self.disconnect_channel.try_recv() {
                if let Some(client) = client.upgrade() {
                    if let Some((index, _)) = self.clients.iter().enumerate().find(|x| *x.1 == client) {
                        disconnect.push(index);
                    }
                }
            }
            
            let mut data_vec  = Vec::new();
            for (index, client) in self.clients.iter_mut().enumerate() {
                if let Some(data) = client.read_all() {
                    if data.len() == 0 {
                        disconnect.push(index);
                    } else {
                        data_vec.push((client.clone(), data));
                    }
                }
            }
            
            for (client, data) in data_vec {
                self.process_request(client, &data);
            }
            
            while let Some(index) = disconnect.pop() {
                let client = self.clients.remove(index);
                self.recv_channel.send((client, Message::Disconnect)).unwrap();
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    pub fn start(port: u16) -> Result<(), io::Error> {
        if port == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Port 0 is not allowed!"));
        }
        
        let recv_channel = mpsc::channel();
        let disconnect_channel = mpsc::channel();
        
        let server = Server {
            socket: TcpListener::bind(format!("127.0.0.1:{}", port))?,
            clients: vec![],
            client_channel: mpsc::channel(),
            recv_channel: recv_channel.0,
            disconnect_channel: disconnect_channel.1
        };
        
        server.socket.set_nonblocking(true).unwrap();
        let tx = server.client_channel.0.clone();        
        let socket = server.socket.try_clone()?;
        
        let receiver = std::thread::spawn(move|| {
            MessageReceiver::new(disconnect_channel.0).run(recv_channel.1);
        });
        
        let acceptor = std::thread::spawn(move || {
            for client in socket.incoming() {
                if let Ok(client) = client {
                    tx.send(Client::new(client)).unwrap();
                }
            }
        });
        
        let thread = std::thread::spawn(move|| {
            server.run();
        });
        
        receiver.join().unwrap();
        thread.join().unwrap();
        acceptor.join().unwrap();
        
        Ok(())
    }
}
