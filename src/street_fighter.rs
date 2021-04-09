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
        make_move(&fighters, &mut pos, m);
        out.push(fighters[pos.y][pos.x].clone());
    }
    out
}

fn make_move(fighters: &Vec<Vec<String>>, pos: &mut Position, m: &Direction) {
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
    if axis.len() == 0 { return; }
    *i = move_along_axis(&axis, *i, pos_dir, wrap);
}

fn move_along_axis(axis: &Vec<String>, i: usize, pos_dir: bool, wrap: bool) -> usize {
    let max = axis.len() - 1;
    let next = match () {
        _ if i == max && pos_dir => if wrap { 0 } else { max },
        _ if i == 0 && !pos_dir => if wrap { max } else { 0 },
        _ if pos_dir => i + 1,
        _ => (i as i64 - 1) as usize,
    };

    if axis[next].len() == 0 {
        move_along_axis(axis, next, pos_dir, wrap)
    } else {
        next
    }
}