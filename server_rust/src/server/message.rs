#[derive(Debug, Clone)]
pub enum Message {
    OK,
    NOK(Option<String>) ,
    CREATE(String),
    JOIN(String),
    READY,
    PUT(usize),
    TAKE(usize),
    MOVE(usize, usize),
    OVER,
    PING,
    PONG,
}

impl Message {
    // TODO: maybe we need &self, but probably not
    pub fn serialize(self) -> Vec<u8> {
        let u32_size = std::mem::size_of::<u32>() as u32;
        
        let mut v = vec![];
        
        match self {
            Message::OK => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(0).to_vec());
            },
            Message::NOK(value) => {
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
            Message::CREATE(username) => {
                v.append(&mut u32::to_be_bytes(2 * u32_size + username.len() as u32).to_vec());
                v.append(&mut u32::to_be_bytes(2).to_vec());
                v.append(&mut username.into_bytes().to_vec());
            },
            Message::JOIN(username) => {
                v.append(&mut u32::to_be_bytes(2 * u32_size + username.len() as u32).to_vec());
                v.append(&mut u32::to_be_bytes(3).to_vec());
                v.append(&mut username.into_bytes().to_vec());
            },
            Message::READY => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(4).to_vec());
            }
            Message::PUT(index) => {
                v.append(&mut u32::to_be_bytes(3*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(5).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            },
            Message::TAKE(index) => {
                v.append(&mut u32::to_be_bytes(3*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(6).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            },
            Message::MOVE(from, to) => {
                v.append(&mut u32::to_be_bytes(4*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(7).to_vec());
                v.append(&mut u32::to_be_bytes(from as u32).to_vec());
                v.append(&mut u32::to_be_bytes(to as u32).to_vec());
            },
            Message::OVER => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(8).to_vec());
            },
            
            Message::PING => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(9).to_vec());
            },
            
            Message::PONG => {
                v.append(&mut u32::to_be_bytes(2*u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(10).to_vec());
            }
        }
        
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
            0 => Some(Self::OK),
            1 => Some(Self::NOK( if data.len() > 0 { Some(String::from_utf8(data.iter().cloned().collect()).ok()?) } else { None } )),
            2 => Some(Self::CREATE(String::from_utf8(data.iter().cloned().collect()).ok()?)),
            3 => Some(Self::JOIN(String::from_utf8(data.iter().cloned().collect()).ok()?)),
            4 => Some(Self::READY),
            5 => if data.len() >= u32_size { Some(Self::PUT(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize)) } else { None },
            6 => if data.len() >= u32_size { Some(Self::TAKE(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize)) } else { None },
            7 => if data.len() >= 2*u32_size { Some(Self::MOVE(u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize, u32::from_be_bytes(data[u32_size..2*u32_size].try_into().ok()?) as usize)) } else { None },
            8 => Some(Self::OVER),
            9 => Some(Self::PING),
            10 => Some(Self::PONG),
            _ => None
        }
    }
}
