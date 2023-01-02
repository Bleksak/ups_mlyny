use crate::{game::{color::Color, board::Board}, machine::State};
use std::str;

pub trait Serializable {
    type Object;
    
    fn serialize(&self) -> Box<[u8]>;
    fn deserialize(bytes: &[u8]) -> Option<Self::Object>;
}

#[derive(Debug, Clone)]
pub enum TextMessage<'a> {
    Mok,
    Nok(Option<String>),
    Create(String),
    Join(String),
    Ready(State, Color, &'a Board, String),
    Put(usize),
    Take(usize),
    Move(usize, usize),
    Over,
    Ping,
    Pong,
    PlayerJoined,
    GameState(State),
    Disconnect,
}

impl<'a> Serializable for TextMessage<'a> {
    type Object = Self;
    
    fn serialize(&self) -> Box<[u8]> {
        match self {
            TextMessage::Mok => "3;OK;".as_bytes().into(),
            TextMessage::Nok(msg) => {
                if let Some(msg) = msg {
                    let len = msg.len() + 4;
                    format!("{len};NOK;{msg}").as_bytes().into()
                } else {
                    "4;NOK;".as_bytes().into()
                }
            },
            TextMessage::Create(username) => format!("{};CREATE;{username}", username.len() + 8).as_bytes().into(),
            TextMessage::Join(username) => format!("{};JOIN;{username}", username.len() + 6).as_bytes().into(),
            TextMessage::Ready(state, color, board, opponent) => {
                let state_string = (state.clone() as u32).to_string();
                let len = opponent.len() + 24 + 1 + state_string.len() + 4 + 5;
                format!("{len};READY;{state_string};{};{};{opponent}", color.serialize(), board.serialize()).as_bytes().into()
            },
            TextMessage::Put(position) => {
                let pos_string = position.to_string();
                format!("{};PUT;{pos_string}", pos_string.len() + 4).as_bytes().into()
            },
            TextMessage::Take(position) => {
                let pos_string = position.to_string();
                format!("{};TAKE;{pos_string}", pos_string.len() + 5).as_bytes().into()
            },
            TextMessage::Move(pos1, pos2) => {
                let pos1_string = pos1.to_string();
                let pos2_string = pos2.to_string();
                
                let len = pos1_string.len() + pos2_string.len() + 6;
                
                let msg = format!("{};MOVE;{};{}", len, pos1_string, pos2_string);
                // println!("SENDING MOVE: {}", msg);
                msg.as_bytes().into()
            },
            TextMessage::Over => "5;OVER;".as_bytes().into(),
            TextMessage::Ping => "5;PING;".as_bytes().into(),
            TextMessage::Pong => "5;PONG;".as_bytes().into(),
            TextMessage::PlayerJoined => "7;JOINED;".as_bytes().into(),
            TextMessage::GameState(state) => {
                let state_string = (state.clone() as u32).to_string();
                let msg = format!("{};STATE;{state_string}", state_string.len() + 6);
                // println!("SENDING STATE: {}", msg);
                msg.as_bytes().into()
            },
            TextMessage::Disconnect => "10;DISCONNECT;".as_bytes().into(),
        }
    }
    
    fn deserialize(bytes: &[u8]) -> Option<Self::Object> {
        let message = str::from_utf8(bytes).ok()?.trim();
        println!("WHOLE_MSG: {message}");
        let whole_msg_len = message.len();
        
        let splitted: Vec<&str> = message.split(';').collect();
        
        let len_str = splitted.get(0)?;
        let len_len = len_str.len() + 1;
        
        let len: usize = len_str.parse().ok()?;
        
        if whole_msg_len - len_len != len {
            return None;
        }
        
        let message = message.trim();
        let splitted: Vec<&str> = message.split(';').collect();
        
        match *splitted.get(1)? {
            "OK" => Some(TextMessage::Mok),
            "NOK" => Some(TextMessage::Nok(None)),
            "CREATE" => Some(TextMessage::Create(splitted.get(2)?.to_string())),
            "JOIN" => Some(TextMessage::Join(splitted.get(2)?.to_string())),
            "PUT" => Some(TextMessage::Put(splitted.get(2)?.parse().ok()?)),
            "TAKE" => Some(TextMessage::Take(splitted.get(2)?.parse().ok()?)),
            "MOVE" => Some(TextMessage::Move(splitted.get(2)?.parse().ok()?, splitted.get(3)?.parse().ok()?)),
            "PING" => Some(TextMessage::Ping),
            "PONG" => Some(TextMessage::Pong),
            "DISCONNECT" => Some(TextMessage::Disconnect),
            _ => None
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Message {
    Mok,
    Nok(Option<String>),
    Create(String),
    Join(String),
    Ready(State, Color, Vec<u8>, String),
    Put(usize),
    Take(usize),
    Move(usize, usize),
    Over,
    Ping,
    Pong,
    PlayerJoined,
    GameState(State),
    Disconnect,
}

#[allow(unused)]
impl Message {
    pub fn serialize(self) -> Vec<u8> {
        let u32_size = std::mem::size_of::<u32>() as u32;

        let mut v = vec![];

        match self {
            Message::Mok => {
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(0).to_vec());
            }
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
            }
            Message::Join(username) => {
                v.append(&mut u32::to_be_bytes(2 * u32_size + username.len() as u32).to_vec());
                v.append(&mut u32::to_be_bytes(3).to_vec());
                v.append(&mut username.into_bytes().to_vec());
            }
            Message::Ready(state, color, mut board, opponent_name) => {
                let size = 1 + board.len() as u32 + 3 * u32_size + opponent_name.len() as u32;
                let color = color.serialize();

                v.append(&mut u32::to_be_bytes(size).to_vec());
                v.append(&mut u32::to_be_bytes(4).to_vec());
                v.append(&mut u32::to_be_bytes(state as u32).to_vec());
                // v.append(&mut u8::to_be_bytes(color).to_vec());
                v.append(&mut board);
                v.append(&mut opponent_name.into_bytes());
            }
            Message::Put(index) => {
                v.append(&mut u32::to_be_bytes(3 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(5).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            }
            Message::Take(index) => {
                v.append(&mut u32::to_be_bytes(3 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(6).to_vec());
                v.append(&mut u32::to_be_bytes(index as u32).to_vec());
            }
            Message::Move(from, to) => {
                v.append(&mut u32::to_be_bytes(4 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(7).to_vec());
                v.append(&mut u32::to_be_bytes(from as u32).to_vec());
                v.append(&mut u32::to_be_bytes(to as u32).to_vec());
            }
            Message::Over => {
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(8).to_vec());
            }

            Message::Ping => {
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(9).to_vec());
            }

            Message::Pong => {
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(10).to_vec());
            }
            Message::PlayerJoined => {
                // println!("Player joined message sent!");
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(11).to_vec());
            }
            Message::Disconnect => {
                v.append(&mut u32::to_be_bytes(2 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(12).to_vec());
            }
            Message::GameState(state) => {
                v.append(&mut u32::to_be_bytes(3 * u32_size).to_vec());
                v.append(&mut u32::to_be_bytes(14).to_vec());
                v.append(&mut u32::to_be_bytes(state as u32).to_vec());
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
        let msg_type_bytes = &bytes[u32_size..2 * u32_size];

        let msg_type_u32 = u32::from_be_bytes(msg_type_bytes.try_into().ok()?);
        let data = &bytes[2 * u32_size..];

        match msg_type_u32 {
            0 => Some(Self::Mok),
            1 => Some(Self::Nok(if data.len() > 0 {
                Some(String::from_utf8(data.iter().cloned().collect()).ok()?)
            } else {
                None
            })),
            2 => Some(Self::Create(
                String::from_utf8(data.iter().cloned().collect()).ok()?,
            )),
            3 => Some(Self::Join(
                String::from_utf8(data.iter().cloned().collect()).ok()?,
            )),
            // 4 => Some(Self::Ready),
            5 => {
                if data.len() >= u32_size {
                    Some(Self::Put(
                        u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize,
                    ))
                } else {
                    None
                }
            }
            6 => {
                if data.len() >= u32_size {
                    Some(Self::Take(
                        u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize,
                    ))
                } else {
                    None
                }
            }
            7 => {
                if data.len() >= 2 * u32_size {
                    Some(Self::Move(
                        u32::from_be_bytes(data[0..u32_size].try_into().ok()?) as usize,
                        u32::from_be_bytes(data[u32_size..2 * u32_size].try_into().ok()?) as usize,
                    ))
                } else {
                    None
                }
            }
            8 => Some(Self::Over),
            9 => Some(Self::Ping),
            10 => Some(Self::Pong),
            // 11 => Some(Self::PlayerJoined( u32::from_be_bytes(data[0..u32_size].try_into().ok()?).try_into().ok()?)),
            _ => None,
        }
    }
}
