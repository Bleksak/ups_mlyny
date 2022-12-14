pub mod client;
pub mod message;
pub mod receiver;

use std::io;
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Weak};
use std::time::Duration;

use self::client::{Client, SocketError};
use self::message::{TextMessage, Serializable};
use self::receiver::MessageReceiver;

pub struct Server<'a> {
    socket: TcpListener,
    clients: Vec<Arc<Client>>,
    client_channel: (Sender<Client>, Receiver<Client>),
    recv_channel: Sender<(Arc<Client>, TextMessage<'a>)>,
    disconnect_channel: (Sender<Weak<Client>>, Receiver<Weak<Client>>),
}

impl Server<'_> {
    fn bad_message(client: Arc<Client>) {
        if let Ok(_) = client.write(&TextMessage::Nok(Some("Bad message".to_string())).serialize()) {
        }
    }
    
    fn process_request(
        recv_channel: Sender<(Arc<Client>, TextMessage)>,
        dc_channel: Sender<Weak<Client>>,
        client: Arc<Client>,
    ) {
        match client.read_all(Some(4096)) {
            Ok(data) => {
                if data.len() != 0 {
                    
                    use std::str;
                    if let Ok(msg_str) = str::from_utf8(&data) {
                        for line in msg_str.lines() {
                            if let Some(message) = TextMessage::deserialize(line.as_bytes()) {
                                println!("{:?}", message);
                                recv_channel.send((client.clone(), message)).unwrap();
                            } else {
                                if client.clone().bad_message() >= 10 {
                                    dc_channel.send(Arc::downgrade(&client)).unwrap();
                                    recv_channel.send((client.clone(), TextMessage::Disconnect)).unwrap();
                                }
                            }
                        }
                    }
                    
                } else {
                    println!("Disconnect!");
                    dc_channel.send(Arc::downgrade(&client)).unwrap();
                    recv_channel.send((client, TextMessage::Disconnect)).unwrap();
                }
            }
            Err(_err @ SocketError::LimitReached) => {
                if client.bad_message() >= 10 {
                    dc_channel.send(Arc::downgrade(&client)).unwrap();
                    recv_channel.send((client, TextMessage::Disconnect)).unwrap();
                }
            }
            Err(_) => {}
        }
    }

    fn run(mut self) {
        loop {
            if let Ok(client) = self.client_channel.1.try_recv() {
                println!("Got new client!");
                let client = Arc::new(client);
                client
                    .lock()
                    .unwrap()
                    .set_nonblocking(true)
                    .expect("Failed to set client nonblocking");
                self.clients.push(client);
            }

            if let Ok(client) = self.disconnect_channel.1.try_recv() {
                if let Some(client) = client.upgrade() {
                    if let Some((index, _)) =
                        self.clients.iter().enumerate().find(|x| *x.1 == client)
                    {
                        self.clients.swap_remove(index);
                    }
                }
            }

            for client in self.clients.clone().into_iter() {
                let channel = self.recv_channel.clone();
                let dc_channel = self.disconnect_channel.0.clone();
                Self::process_request(channel, dc_channel, client);
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn start(address: String, port: u16) -> Result<(), io::Error> {
        if port == 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Port 0 is not allowed!",
            ));
        }

        let recv_channel = mpsc::channel();
        let disconnect_channel = mpsc::channel();
        let dc_sender = disconnect_channel.0.clone();

        let server = Server {
            socket: TcpListener::bind(format!("{}:{}", address, port))?,
            clients: vec![],
            client_channel: mpsc::channel(),
            recv_channel: recv_channel.0,
            disconnect_channel,
        };

        server.socket.set_nonblocking(true).unwrap();
        let tx = server.client_channel.0.clone();
        let socket = server.socket.try_clone()?;

        let receiver = std::thread::spawn(move || {
            MessageReceiver::new(dc_sender).run(recv_channel.1);
        });

        let acceptor = std::thread::spawn(move || {
            for client in socket.incoming() {
                if let Ok(client) = client {
                    tx.send(Client::new(client)).unwrap();
                }
                
                std::thread::sleep(Duration::from_millis(100));
            }
        });

        let thread = std::thread::spawn(move || {
            server.run();
        });
        
        receiver.join().unwrap();
        thread.join().unwrap();
        acceptor.join().unwrap();

        Ok(())
    }
}
