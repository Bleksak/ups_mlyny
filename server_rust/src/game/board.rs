use std::sync::Mutex;
use super::color::Color;

pub struct Board {
    board: Mutex<[Color; 24]>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: Mutex::new([Color::Neutral; 24])
        }
    }
    
    pub fn put(&self, index: usize, color: Color) -> Result<(), String> {
        let mut guard = self.board.lock().unwrap();
        let mut field = guard.get_mut(index).ok_or("Index out of range".to_string())?;
        if *field != Color::Neutral {
            return Err("Field is already taken".to_string());
        }
        
        *field = color;
        
        Ok(())
    }
}
