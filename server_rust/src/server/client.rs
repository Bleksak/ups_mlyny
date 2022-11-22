use std::{net::TcpStream, os::unix::prelude::AsRawFd, hash::Hash};
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

use crate::machine::Machine;

pub struct Client(TcpStream, Option<Machine>);

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self (stream, None)
    }
    
    pub fn set_machine(&mut self, machine: Machine) {
        self.1 = Some(machine);
    }
    
    // pub fn machine(&self) -> &Machine {
    //     self.1.as_ref().unwrap()
    // }
    
    pub fn machine_mut(&mut self) -> &mut Machine {
        self.1.as_mut().unwrap()
    }
    
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), io::Error> {
        self.write_all(bytes)
    }
    
    pub fn read_all(&mut self) -> Option<Vec<u8>>{
        let mut data : Vec<u8> = Vec::with_capacity(512);
        data.resize(512, 0);
        
        let mut read_bytes = 0;
        
        let read = self.read(&mut data[..]);
        
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
            let read = self.read(&mut data[read_bytes..read_bytes+512]);
            
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
        self.as_raw_fd()
    }
    
    #[cfg(target_os="windows")]
    pub fn sock_fd(&self) -> i32 {
        self.as_raw_socket() as i32
    }
}

impl Deref for Client {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self::new(self.try_clone().unwrap())
    }
}

#[cfg(target_os="linux")]
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.as_raw_fd() == other.as_raw_fd()
    }
}

#[cfg(target_os="linux")]
impl Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.as_raw_fd());
        state.finish();
    }
}

#[cfg(target_os="windows")]
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.as_raw_socket() == other.as_raw_socket()
    }
}

#[cfg(target_os="windows")]
impl Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_raw_socket());
        state.finish();
    }
}

impl Eq for Client {}
