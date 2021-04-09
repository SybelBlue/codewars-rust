use std::collections::HashMap;

use self::{Instruction::*, Value::*};

type Register = char;

#[derive(Debug, Clone)]
enum Instruction {
    Mov(Register, Value),
    Inc(Register),
    Dec(Register),
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

fn access_register<'a>(registers: &'a mut Vec<Option<i64>>, r: &char) -> &'a mut i64 {
    registers[*r as usize - 'a' as usize].as_mut().unwrap_or_else(|| panic!(format!("Accessed unintialized register '{}'", r)))
}

pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = vec![None; 26]; // None for 'a'..='z'
    let mut parsed_program = vec![None; program.len()];
    let mut pc = 0;
    while pc < program.len() {
        let instruction = parsed_program[pc].get_or_insert_with(|| {
            let line = program[pc];
            parse(line).map_err(|msg| panic!(format!("error \"{}\" at \n| {}: {}", msg, pc, line))).unwrap()
        });
        
        match instruction {
            Mov(r, v) => registers[*r as usize - 'a' as usize] = Some(v.evaluate(&mut registers)),
            Inc(r) => *access_register(&mut registers, r) += 1,
            Dec(r) => *access_register(&mut registers, r) -= 1,
            Jnz(u, v) => if u.evaluate(&mut registers) != 0 {
                pc = (pc as i64 + v.evaluate(&mut registers)) as usize;
                continue;
            },
        }
        pc += 1;
    }
    
    registers.into_iter().enumerate()
        .filter_map(|(i, v)| v.map(|n| (i, n)))
        .map(|(i, v)| (format!("{}", (i as u8 + 'a' as u8) as char), v)).collect()
}

fn parse(line: &str) -> Result<Instruction, String> {
    let s = String::from(line);
    let mut itr = s.split_ascii_whitespace();
    let out = match itr.next() {
        Some("mov") => Ok(Mov(parse_register(itr.next())?, parse_value(itr.next())?)),
        Some("inc") => Ok(Inc(parse_register(itr.next())?)),
        Some("dec") => Ok(Dec(parse_register(itr.next())?)),
        Some("jnz") => Ok(Jnz(parse_value(itr.next())?, parse_value(itr.next())?)),
        Some(x) => Err(format!("'{}' not recognized as instruction", x)),
        None => Err(String::from("Empty line")),
    }?;
    if let Some(v) = itr.next() {
        Err(format!("Expected end of line after {:?}, recieved {} instead", out, v))
    } else {
        Ok(out)
    }
}

fn parse_value(v_str: Option<&str>) -> Result<Value, String> {
    let s = v_str.ok_or_else(|| String::from("Expected value, got end of line"))?;
    s.parse::<i64>().map_or_else(|_| parse_register(v_str).map(Reg), |n| Ok(Num(n)))
}

fn parse_register(v_str: Option<&str>) -> Result<Register, String> {
    let s = v_str.ok_or_else(|| String::from("Expected value, got end of line"))?;
    if s.len() == 1 {
        let c = s.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            return Ok(c);
        }
    }
    Err(format!("Bad register '{}', requires single ascii lowercase letter", s))
}
