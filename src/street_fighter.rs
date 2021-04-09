#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// //////////////////////// CANNOT CHANGE ABOVE LINE /////////////////////////

fn super_street_fighter_selection(fighters: &[&[&str]], position: Position, moves: &[Direction]) 
        -> Vec<String> {
    todo!()
}

fn make_move(fighters: &[&[&str]], position: Position, m: Direction) -> Position {
    let row = fighters[position.y];
    match direction {
        
    }
}