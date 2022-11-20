use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum Message {
    OK,
    NOK ,
    INIT,
    CREATE(String),
    JOIN(String),
    PUT(usize),
    MOVE(usize, usize),
    TAKE(usize),
}

impl Message {
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        let u32_size = std::mem::size_of::<u32>();
        
        let msg_len_bytes = &bytes[0..u32_size];
        let msg_type_bytes = &bytes[u32_size..2*u32_size];
        
        
        // bytes.chunks(std::mem::size_of::<u32>());
        
        
        Some(Self::OK)
    }
}
