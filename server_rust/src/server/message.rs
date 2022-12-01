use crate::{machine::State, game::color::Color};

#[derive(Debug, Clone)]
pub enum Message {
    Mok,
    Nok(Option<String>) ,
    Create(String),
    Join(String),
    Ready(State, Color, Vec<u8>),
    Put(usize),
    Take(usize),
    Move(usize, usize),
    Over,
    Ping,
    Pong,
    PlayerJoined,
}

impl Message {
    pub fn serialize(self) -> Vec<u8> {
        let u32_size = std::mem::size_of::<u32>() as u32;
        
        let mut v = vec![];
        
        match self {
            Message::Mok => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(0).to_vec());
            },
            Message::Nok(value) => {
                if let Some(value) = value {
                    let msg_len = 2 * u32_size + value.len() as u32;
                    v.append(&mut u32::to_be_bytes(msg_len).to_vec());
                    v.append(&mut u32::to_be_bytes(1).to_vec());
                    v.append(&mut value.into_bytes().to_vec());
                } else {
                    v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                    v.append(&mut u32::to_be_bytes(1).to_vec());
                }
            }
            Message::Create(username) => {
                v.append(&mut u32::to_be_bytes(2 * u32_size + username.len() as u32).to_vec());
                v.append(&mut u32::to_be_bytes(2).to_vec());
                v.append(&mut username.into_bytes().to_vec());
            },
            Message::Join(username) => {
                v.append(&mut u32::to_be_bytes(2 * u32_size + username.len() as u32).to_vec());
                v.append(&mut u32::to_be_bytes(3).to_vec());
                v.append(&mut username.into_bytes().to_vec());
            },
            Message::Ready(state, color, mut board) => {
                let size = 1 + board.len() as u32 + 3 * u32_size;
                let color = color.serialize();
                
                v.append(&mut u32::to_be_bytes(size).to_vec());
                v.append(&mut u32::to_be_bytes(4).to_vec());
                v.append(&mut u32::to_be_bytes(state as u32).to_vec());
                v.append(&mut u8::to_be_bytes(color).to_vec());
                v.append(&mut board);
            }
            Message::Put(index) => {
                v.append(&mut u32::to_be_bytes(3*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(5).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            },
            Message::Take(index) => {
                v.append(&mut u32::to_be_bytes(3*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(6).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            },
            Message::Move(from, to) => {
                v.append(&mut u32::to_be_bytes(4*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(7).to_vec());
                v.append(&mut u32::to_be_bytes(from as u32).to_vec());
                v.append(&mut u32::to_be_bytes(to as u32).to_vec());
            },
            Message::Over => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(8).to_vec());
            },
            
            Message::Ping => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(9).to_vec());
            },
            
            Message::Pong => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(10).to_vec());
            },
            Message::PlayerJoined => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(11).to_vec());
            }
        }
        
        println!("will send: {} bytes ", v.len());
        v
    }
    
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        let u32_size = std::mem::size_of::<u32>();
        
        if bytes.len() < 2 * u32_size {
            return None;
        }
        
        let msg_len_bytes = &bytes[0..u32_size];
        
        let msg_len = u32::from_be_bytes(msg_len_bytes.try_into().ok()?) as usize;
        
        if msg_len != bytes.len() {
            return None;
        }
        let msg_type_bytes = &bytes[u32_size..2*u32_size];
        
        let msg_type_u32= u32::from_be_bytes(msg_type_bytes.try_into().ok()?);
        let data = &bytes[2*u32_size..];
        
        match msg_type_u32 {
            0 => Some(Self::Mok),
            1 => Some(Self::Nok( if data.len() > 0 { Some(String::from_utf8(data.iter().cloned().collect()).ok()?) } else { None } )),
            2 => Some(Self::Create(String::from_utf8(data.iter().cloned().collect()).ok()?)),
            3 => Some(Self::Join(String::from_utf8(data.iter().cloned().collect()).ok()?)),
            // 4 => Some(Self::Ready),
            5 => if data.len() >= u32_size { Some(Self::Put(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize)) } else { None },
            6 => if data.len() >= u32_size { Some(Self::Take(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize)) } else { None },
            7 => if data.len() >= 2*u32_size { Some(Self::Move(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize, u32::from_be_bytes(data[u32_size..2*u32_size].try_into().ok()?) as usize)) } else { None },
            8 => Some(Self::Over),
            9 => Some(Self::Ping),
            10 => Some(Self::Pong),
            // 11 => Some(Self::PlayerJoined( u32::from_be_bytes(data[0..u32_size].try_into().ok()?).try_into().ok()?)),
            _ => None
        }
    }
}
