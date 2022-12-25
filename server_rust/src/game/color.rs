#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Neutral,
    Red,
    Blue,
}

impl Color {
    pub fn serialize(&self) -> char {
        match self {
            Self::Neutral => '0',
            Self::Red => '1',
            Self::Blue => '2',
        }
    }
}
