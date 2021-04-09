use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
    Mov(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
}

#[derive(Debug, Clone)]
enum Value {
    Reg(char),
    Num(i64),
}

impl Value {
    pub fn evaluate(&self, registers: &mut Vec<Option<i64>>) -> i64 {
        match self {
            Reg(c) => *access_register(registers, c),
            Num(v) => *v,
        }
    }
}

use self::{Instruction::*, Value::*};

fn access_register<'a>(registers: &'a mut Vec<Option<i64>>, c: &char) -> &'a mut i64 {
    if let Some(v) = registers[*c as usize - 'a' as usize].as_mut() {
        v
    } else {
        panic!(format!("Accessed unintialized register '{}'", c))
    }
}

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = vec![None; 26];
    let mut parsed_program = vec![None; program.len()];
    let mut program_counter = 0;
    while program_counter < program.len() {
        let instruction = parsed_program[program_counter].get_or_insert_with(|| {
            let line = program[program_counter];
            interpret(line).map_err(|msg| panic!(format!("error {} at {}: {}", msg, program_counter, line))).unwrap()
        });
        
        match instruction {
            Mov(Reg(c), v) => registers[*c as usize - 'a' as usize] = Some(v.evaluate(&mut registers)),
            Mov(Num(n), _) => panic!(format!("mov requires destination to be a register, not '{}'", n)),
            Inc(Reg(c)) => *access_register(&mut registers, c) += 1,
            Dec(Reg(c)) => *access_register(&mut registers, c) -= 1,
            Jnz(u, v) => {
                if u.evaluate(&mut registers) != 0 {
                    program_counter = (program_counter as i64 + v.evaluate(&mut registers)) as usize;
                    continue;
                }
            },
            _ => panic!("inc/dec requires target to be a register")
        }
        program_counter += 1;
    }
    
    registers.into_iter().enumerate()
        .filter_map(|(i, v)| v.map(|n| (i, n)))
        .map(|(i, v)| (format!("{}", (i as u8 + 'a' as u8) as char), v)).collect()
}

fn interpret(line: &str) -> Result<Instruction, String> {
    let s = String::from(line);
    let mut itr = s.split_ascii_whitespace();
    let out = match itr.next() {
        Some("mov") => Ok(Mov(parse_value(itr.next())?, parse_value(itr.next())?)),
        Some("inc") => Ok(Inc(parse_value(itr.next())?)),
        Some("dec") => Ok(Dec(parse_value(itr.next())?)),
        Some("jnz") => Ok(Jnz(parse_value(itr.next())?, parse_value(itr.next())?)),
        Some(x) => Err(format!("'{}' not recognized as instruction", x)),
        None => Err(String::from("Empty line!")),
    }?;
    if let Some(v) = itr.next() {
        Err(format!("Expected end of line after {:?}, recieved {} instead", out, v))
    } else {
        Ok(out)
    }
}

fn parse_value(v_str: Option<&str>) -> Result<Value, String> {
    if let Some(s) = v_str {
        match s.parse::<i64>() {
            Ok(n) => Ok(Num(n)),
            _ => {
                if s.len() == 1 {
                    let c = s.chars().next().unwrap();
                    if c.is_ascii_lowercase() {
                        return Ok(Reg(c));
                    }
                }
                Err(format!("Bad register '{}', requires single ascii lowercase letter", s))
            },
        }
    } else {
        Err(String::from("Expected value, got end of line"))
    }
}

fn main() {
    let program = vec![
        "mov c 12",
        "mov b 0",
        "mov a 200",
        "dec a",
        "inc b",
        "jnz a -2",
        "dec c",
        "mov a b",
        "jnz c -5",
        "jnz 0 1",
        "mov c a",
    ];
    println!("{:#?}", simple_assembler(program));
    println!("{:#?}", simple_assembler(vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"]));
}
