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

pub fn super_street_fighter_selection(fighters: &[&[&str]], position: Position, moves: &[Direction]) 
        -> Vec<String> {
    let fighters : Vec<Vec<String>> = {
        let mut out = Vec::new();
        for r in fighters {
            let mut row = Vec::new();
            for s in *r {
                row.push(String::from(*s));
            }
            out.push(row);
        }
        out
    };
    let mut pos = position.clone();
    moves.iter().map(|m| {
        make_move(&fighters, &mut pos, m);
        fighters[pos.y][pos.x].clone()
    }).collect()
}

fn make_move(fighters: &Vec<Vec<String>>, pos: &mut Position, m: &Direction) {
    use Direction::*;

    let (shifting, fixed, wrap) = match m {
        Down | Up    => (&mut pos.y, pos.x, false),
        Left | Right => (&mut pos.x, pos.y, true),
    };
    
    let axis = match m {
        Down | Up    => fighters.into_iter().map(|a| a[fixed].clone()).collect(),
        Left | Right => fighters[fixed].clone(),
    };
    
    let pos_dir = *m == Down || *m == Right;
    
    if axis.len() == 0 { return; }
    
    *shifting = move_along_axis(&axis, *shifting, pos_dir, wrap);
}

fn move_along_axis(axis: &Vec<String>, i: usize, pos_dir: bool, wrap: bool) -> usize {
    let max = axis.len() - 1;
    let mut next = i;
    println!("start move: {:?} @ {}", axis, axis[next]);
    loop {
        let at_boundary = next == if pos_dir { max } else { 0 };
        
        if !wrap && at_boundary {
            let n = if axis[next].len() == 0 { i } else { next };
            println!("wrap boot {} -> {} ({})", i, n, axis[next]);
            return n;
        }
        
        next = if at_boundary {
            if next == 0 { max } else { 0 }
        } else {
            if pos_dir { next + 1 } else { (next as i64 - 1) as usize }
        };

        if axis[next].len() != 0 {
            println!("ok boot {} -> {} ({})", i, next, axis[next]);
            return next;
        }
    }
}