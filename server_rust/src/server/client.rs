use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

#[derive(Debug)]
pub struct Client(Mutex<TcpStream>, Mutex<usize>);

#[derive(PartialEq)]
pub enum SocketError {
    WouldBlock,
    LimitReached,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self(Mutex::new(stream), Mutex::new(0))
    }

    pub fn bad_message(&self) -> usize {
        let mut guard = self.1.lock().unwrap();
        *guard += 1;
        *guard
    }

    pub fn write(&self, bytes: &[u8]) -> Result<(), io::Error> {
        let mut lock = self.lock().unwrap();
        lock.write_all(bytes)?;
        lock.flush()
    }

    pub fn read_all(&self, limit: Option<usize>) -> Result<Vec<u8>, SocketError> {
        let mut data: Vec<u8> = Vec::with_capacity(512);
        data.resize(512, 0);

        let mut read_bytes = 0;

        let read = self.lock().unwrap().read(&mut data[..]);

        if let Err(err) = read {
            match err.kind() {
                io::ErrorKind::WouldBlock => {
                    return Err(SocketError::WouldBlock);
                }

                _ => {}
            }
        } else {
            read_bytes += read.unwrap();
        }

        loop {
            data.resize(read_bytes + 512, 0);
            if let Some(limit) = limit {
                if read_bytes > limit {
                    return Err(SocketError::LimitReached);
                }
            }
            let read = self
                .lock()
                .unwrap()
                .read(&mut data[read_bytes..read_bytes + 512]);

            if let Err(err) = read {
                match err.kind() {
                    io::ErrorKind::WouldBlock => {
                        data.truncate(read_bytes);
                        return Ok(data);
                    }

                    _ => {}
                }
            } else {
                let read = read.unwrap();
                if read == 0 {
                    data.truncate(read_bytes);
                    return Ok(data);
                }
                read_bytes += read;
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn sock_fd(&self) -> i32 {
        use std::os::unix::prelude::AsRawFd;
        self.lock().unwrap().as_raw_fd()
    }

    #[cfg(target_os = "windows")]
    pub fn sock_fd(&self) -> i32 {
        use std::os::windows::prelude::AsRawSocket;
        self.lock().unwrap().as_raw_socket() as i32
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
        self.sock_fd() == other.sock_fd()
    }
}
