use std::{sync::{Mutex, MutexGuard}, collections::HashSet};
use once_cell::sync::Lazy;
use super::{color::Color, GameError};

#[derive(Debug)]
pub struct Board {
    board: Mutex<[Color; 24]>,
}

static NEIGHBORS: Lazy<Vec<(Vec<usize>, Vec<usize>)>> = Lazy::new(|| {
    vec![
        (vec![1], vec![9]), // 0
        (vec![0, 2], vec![4]), // 1
        (vec![1], vec![14]), // 2
        (vec![4], vec![10]), // 3
        (vec![3, 5], vec![1, 7]), // 4
        (vec![4], vec![13]), // 5
        (vec![7], vec![11]), // 6
        (vec![6, 8], vec![4]), // 7
        (vec![7], vec![12]), // 8
        (vec![10], vec![0, 21]), // 9
        (vec![9, 11], vec![3, 18]), // 10
        (vec![10], vec![6, 15]), // 11
        (vec![13], vec![8, 17]), // 12
        (vec![12, 14], vec![5, 20]), // 13
        (vec![13], vec![2, 23]), // 14
        (vec![16], vec![11]), // 15
        (vec![15, 17], vec![19]), // 16
        (vec![16], vec![12]), // 17
        (vec![19], vec![10]), // 18
        (vec![18, 20], vec![16, 22]), // 19
        (vec![19], vec![13]), // 20
        (vec![22], vec![9]), // 21
        (vec![21, 23], vec![19]), // 22
        (vec![22], vec![14]), // 23
    ]
});

impl Board {
    pub fn new() -> Self {
        Self {
            board: Mutex::new([Color::Neutral; 24])
        }
    }
    
    pub fn serialize(&self) -> Vec<u8> {
        self.board.lock().unwrap().iter().map(|col| col.serialize()).collect()
    }
    
    pub fn put(&self, index: usize, color: Color) -> Result<(), GameError> {
        let mut guard = self.board.lock().unwrap();
        let field = guard.get_mut(index).ok_or(GameError::BadPosition)?;
        
        if *field != Color::Neutral {
            return Err(GameError::FieldTaken);
        }
        
        *field = color;
        
        Ok(())
    }
    
    pub fn get(&self, index: usize) -> Result<Color, GameError> {
        Ok(*self.board.lock().unwrap().get(index).ok_or(GameError::BadPosition)?)
    }
    
    /// Checks whether mill was formed vertically
    pub fn check_mill_vertical(&self, start: usize, guard: Option<MutexGuard<[Color; 24]>>) -> bool {
        if let Some(guard) = guard {
            
            let color = *guard.get(start).unwrap();
            
            let mut stack = vec![start];
            let mut set = HashSet::new();
            set.insert(start);
            
            while let Some(item) = stack.pop() {
                for neighbor in NEIGHBORS.get(item).unwrap().1.iter() {
                    if set.contains(neighbor) {
                        continue;
                    }
                    
                    if *guard.get(*neighbor).unwrap() == color {
                        set.insert(*neighbor);
                        stack.push(*neighbor);
                    }
                }
            }
            
            set.len() == 3
        } else {
            self.check_mill_vertical(start, Some(self.board.lock().unwrap()))
        }
    }
    
    /// Checks whether mill was formed horizontally
    pub fn check_mill_horizontal(&self, start: usize, guard: Option<MutexGuard<[Color; 24]>>) -> bool {
        if let Some(guard) = guard {
            
            let color = *guard.get(start).unwrap();
            
            let mut stack = vec![start];
            let mut set = HashSet::new();
            set.insert(start);
            
            while let Some(item) = stack.pop() {
                for neighbor in NEIGHBORS.get(item).unwrap().0.iter() {
                    if set.contains(neighbor) {
                        continue;
                    }
                    
                    if *guard.get(*neighbor).unwrap() == color {
                        set.insert(*neighbor);
                        stack.push(*neighbor);
                    }
                }
            }
            
            set.len() == 3
        } else {
            self.check_mill_horizontal(start, Some(self.board.lock().unwrap()))
        }
    }
    
    /// Checks if the board is still playable for the next player
    pub fn check_draw(&self, last_color: Color) -> bool {
        let guard = self.board.lock().unwrap();
        let color = if last_color == Color::Red { Color::Blue } else { Color::Red };
        
        for (index, item) in guard.iter().enumerate() {
            if *item != color {
                continue;
            }
            
            for neighbor_x in NEIGHBORS[index].0.iter() {
                if *guard.get(*neighbor_x).unwrap() == Color::Neutral {
                    return false;
                }
            }
            
            for neighbor_x in NEIGHBORS[index].0.iter() {
                if *guard.get(*neighbor_x).unwrap() == Color::Neutral {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Moves color from one position to another one
    pub fn mmove(&self, (from, to): (usize, usize), jump_allowed: bool) -> Result<(), GameError> {
        let mut guard = self.board.lock().unwrap();
        println!("jump allowed? {}", jump_allowed);
        if NEIGHBORS.len() > from && NEIGHBORS.len() > to {
            if !jump_allowed && NEIGHBORS[from].0.contains(&to) && NEIGHBORS[from].1.contains(&to) {
                return Err(GameError::CannotMove);
            }
        } else {
            return Err(GameError::BadPosition);
        }
        
        let color = {
            let c = guard.get_mut(from).ok_or(GameError::BadPosition)?;
            let color = *c;
            *c = Color::Neutral;
            color
        };
        
        let c = guard.get_mut(to).ok_or(GameError::BadPosition)?;
        *c = color;
        
        Ok(())
    }
    
    pub fn take(&self, pos: usize) -> Result<(), GameError> {
        let mut guard = self.board.lock().unwrap();
        let c = guard.get_mut(pos).ok_or(GameError::BadPosition)?;
        *c = Color::Neutral;
        Ok(())
    }
}
