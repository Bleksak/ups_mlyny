mod message;

use std::io::{self, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::os::unix::prelude::AsFd;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Server {
    socket: TcpListener,
    clients: Vec<TcpStream>,
    channel: (Sender<TcpStream>, Receiver<TcpStream>),
}

impl Server {
    pub fn new(port: u16) -> Result<Self, io::Error> {
        if port == 0 {
            Err(io::Error::new(io::ErrorKind::Other, "Port 0 is not allowed!"))
        } else {
            Ok(Self {
                socket: TcpListener::bind(format!("127.0.0.1:{}", port))?,
                clients: vec![],
                channel: mpsc::channel()
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
                        println!("some dada");
                        return Some(data)
                    },
                
                    _ => {
                        println!("no data?");
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
    
    fn process_request(&self, client: &mut TcpStream, data: &Vec<u8>) {
        println!("read some data!");
        println!("{:#?}", data);
    }
    
    fn run(mut self) {
        loop {
            if let Ok(client) = self.channel.1.try_recv() {
                println!("Got new client!");
                client.set_nonblocking(true).expect("Failed to set client nonblocking");
                self.clients.push(client);
            }
            
            let mut disconnect = vec![];
            
            let mut data_vec: Vec<(TcpStream, Vec<u8>)> = Vec::new();
            for (index, client) in self.clients.iter_mut().enumerate() {
                if let Some(data) = Self::read_all(client) {
                    if data.len() == 0 {
                        disconnect.push(index);
                    } else {
                        data_vec.push((client.try_clone().unwrap(), data));
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
            
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
    
    pub fn start(self) {
        self.socket.set_nonblocking(true).unwrap();
        
        let socket = self.socket.try_clone().unwrap();
        let tx = self.channel.0.clone();

        let acceptor = std::thread::spawn(move || {
            for client in socket.incoming() {
                if let Ok(client) = client {
                    tx.send(client).unwrap();
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



// f.seekp( N , ios_base::beg);