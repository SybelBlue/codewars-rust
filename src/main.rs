use std::collections::HashMap;

type Registers = HashMap<String, i64>;

type ParseResult<T> = Result<T, String>;

#[derive(Debug, Clone)]
enum Instruction {
    Mov(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
}

#[derive(Debug, Clone)]
enum Value {
    Reg(String),
    Num(i64),
}

impl Value {
    pub fn evaluate(&self, registers: &Registers) -> i64 {
        match self {
            Reg(s) => *registers.get(s).expect(format!("Accessed unintialized register '{}'", s).as_str()),
            Num(v) => *v,
        }
    }
}

use self::{Instruction::*, Value::*};

fn simple_assembler(program: Vec<&str>) -> Registers {
    let mut registers = Registers::new();
    let mut parsed_program = vec![None; program.len()];
    let mut program_counter = 0;
    while program_counter < program.len() {
        let instruction = parsed_program[program_counter].get_or_insert_with(|| {
            let line = program[program_counter];
            interpret(line).map_err(|msg| panic!(format!("error {} at {}: {}", msg, program_counter, line))).unwrap()
        });
        
        match instruction {
            Mov(Reg(u), v) => { registers.insert(u.clone(), v.evaluate(&registers)); },
            Mov(u, v) => panic!(format!("mov requires destination to be a register, not {:?} and {:?}", u, v)),
            Inc(Reg(key)) => {
                let x = registers.get_mut(key).expect(format!("Accessed unintialized register '{}'", key).as_str());
                *x += 1;
            },
            Dec(Reg(key)) => {
                let x = registers.get_mut(key).expect(format!("Accessed unintialized register '{}'", key).as_str());
                *x -= 1;
            },
            Jnz(u, v) => {
                if u.evaluate(&registers) != 0 {
                    program_counter = (program_counter as i64 + v.evaluate(&registers)) as usize;
                    continue;
                }
            },
            _ => panic!("inc/dec requires target to be a register")
        }
        program_counter += 1;
    }
    registers
}

fn interpret(line: &str) -> ParseResult<Instruction> {
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

fn parse_value(v_str: Option<&str>) -> ParseResult<Value> {
    if let Some(s) = v_str {
        Ok(match s.parse::<i64>() {
            Ok(n) => Num(n),
            Err(_) => Reg(String::from(s)),
        })
    } else {
        Err(String::from("expected value, got end of line"))
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
