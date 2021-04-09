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
    let mut out = Vec::with_capacity(moves.len() + 1);
    out.push(fighters[pos.y][pos.x].clone());
    for m in moves {
        if make_move(&fighters, &mut pos, m) {
            out.push(fighters[pos.y][pos.x].clone());
        }
    }
    out
}

fn make_move(fighters: &Vec<Vec<String>>, pos: &mut Position, m: &Direction) -> bool {
    use Direction::*;
    let (i, wrap) = match m {
        Down | Up => (&mut pos.y, false),
        Left | Right => (&mut pos.x, true),
    };
    let (axis, pos_dir) = match m {
        Up => (fighters.into_iter().map(|a| a[*i].clone()).collect(), true),
        Down => (fighters.into_iter().map(|a| a[*i].clone()).collect(), false),
        Right => (fighters[*i].clone(), true),
        Left => (fighters[*i].clone(), false),
    };
    if axis.len() == 0 { return false; }
    let next = move_along_axis(&axis, *i, pos_dir, wrap);
    let old = *i;
    *i = next;
    old == next
}

fn move_along_axis(axis: &Vec<String>, i: usize, pos_dir: bool, wrap: bool) -> usize {
    let max = axis.len() - 1;
    let mut next = i;
    loop {
        let at_boundary = (i == max && pos_dir) || (i == 0 && !pos_dir);
        
        if !wrap && at_boundary {
            return if axis[next].len() == 0 { i } else { next };
        }
        
        next = if at_boundary {
            if next == 0 { max } else { 0 }
        } else {
            if pos_dir { next + 1 } else { (next as i64 - 1) as usize }
        };

        if axis[next].len() != 0 {
            return next;
        }
    }
}