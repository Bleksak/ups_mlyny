#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Neutral,
    Red,
    Blue
}

impl Color {
    pub fn serialize(&self) -> u8 {
        match self {
            Self::Neutral => 0,
            Self::Red => 1,
            Self::Blue => 2,
        }
    }
    
    pub fn deserialize(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Neutral),
            1 => Some(Self::Red),
            2 => Some(Self::Blue),
            _ => None
        }
    }
}
