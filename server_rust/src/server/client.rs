use std::{net::TcpStream, os::unix::prelude::AsRawFd, hash::Hash};


pub struct Client(TcpStream);

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self { 0: stream }
    }
    
    pub fn stream(&self) -> &TcpStream {
        &self.0
    }
    
    pub fn stream_mut(&mut self) -> &mut TcpStream {
        &mut self.0
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self::new(self.stream().try_clone().unwrap())
    }
}

#[cfg(target_os="linux")]
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.stream().as_raw_fd() == other.stream().as_raw_fd()
    }
}

#[cfg(target_os="linux")]
impl Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.stream().as_raw_fd());
        state.finish();
    }
}

#[cfg(target_os="windows")]
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.stream().as_raw_socket() == other.stream().as_raw_socket()
    }
}

#[cfg(target_os="windows")]
impl Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.stream().as_raw_socket());
        state.finish();
    }
}
