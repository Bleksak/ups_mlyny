use std::{net::TcpStream, time::Duration, io::{Read, Write}};

fn receiver(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        if let Ok(read) = stream.read(&mut buf) {
            if read == 0 {
                return;
            }
            for item in buf {
                print!("{:02x}, ", item);
            }
        }
        
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let ip = std::env::args().nth(1).expect("IP undefined");
    let port: u16 = std::env::args().nth(2).expect("Port undefined").parse().unwrap();
    let mut stream = TcpStream::connect(format!("{ip}:{port}")).expect("Failed to connect");
    
    let stream_clone = stream.try_clone().unwrap();
    
    std::thread::spawn(move || {
        receiver(stream_clone);
    });
    
    println!("U: unsigned int");
    println!("L: unsigned long");
    println!("B: unsigned char");
    println!("S: string");
    println!("K: send message");
        
    let mut buffer = vec![];
    
    loop {
        
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        
        if line.as_str().trim().is_empty() {
            continue;
        }
        
        let split: Vec<&str> = line.splitn(2, ' ').collect();
        let msg = split[1].trim();
        
        match split[0] {
            "U" => {
                let data:u32 = msg.parse().unwrap();
                // stream.write_all(&data.to_be_bytes()).unwrap();
                for b in data.to_be_bytes() {
                    buffer.push(b);
                }
            }
            "B" => {
                let data:u8 = msg.parse().unwrap();
                for b in data.to_be_bytes() {
                    buffer.push(b);
                }
            },
            "L" => {
                let data:u64 = msg.parse().unwrap();
                for b in data.to_be_bytes() {
                    buffer.push(b);
                }
            }
            "S" => {
                for b in msg.as_bytes() {
                    buffer.push(*b);
                }
            },
            "K" => {
                stream.write_all(&buffer).unwrap();
            }
            _ => {println!("invalid type")}
        }
        
        std::thread::sleep(Duration::from_millis(100));
    }
}
