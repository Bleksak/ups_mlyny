use std::{net::TcpStream, os::unix::prelude::AsRawFd};
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

use crate::machine::Machine;

pub struct Client(Mutex<TcpStream>, Mutex<Machine>);

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self (Mutex::new(stream), Mutex::new(Machine::new()))
    }
    
    pub fn machine(&self) -> &Mutex<Machine> {
        &self.1
    }
    
    pub fn write(&self, bytes: &[u8]) -> Result<(), io::Error> {
        self.lock().unwrap().write_all(bytes)
    }
    
    pub fn read_all(&self) -> Option<Vec<u8>>{
        let mut data : Vec<u8> = Vec::with_capacity(512);
        data.resize(512, 0);
        
        let mut read_bytes = 0;
        
        let read = self.lock().unwrap().read(&mut data[..]);
        
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
            let read = self.lock().unwrap().read(&mut data[read_bytes..read_bytes+512]);
            
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
    
    #[cfg(target_os="linux")]
    pub fn sock_fd(&self) -> i32 {
        self.lock().unwrap().as_raw_fd()
    }
    
    #[cfg(target_os="windows")]
    pub fn sock_fd(&self) -> i32 {
        self.as_raw_socket() as i32
    }
}

impl Deref for Client {
    type Target = Mutex<TcpStream>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        // TODO: this may be a dead lock on a match
        self.lock().unwrap().as_raw_fd() == other.lock().unwrap().as_raw_fd()
    }
}
