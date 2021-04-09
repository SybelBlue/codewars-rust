use std::collections::HashMap;

type Registers = HashMap<String, i64>;
type Program = HashMap<usize, Instr>;

type ParseResult<T> = Result<T, String>;

#[derive(Debug)]
enum Instr {
    Mov(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value),
}

#[derive(Debug)]
enum Value {
    Reg(String),
    Num(i64),
}

fn simple_assembler(program: Vec<&str>) -> Registers {
    let mut registers = Registers::new();
    let mut parsed_program = Program::with_capacity(program.len());
    let mut program_counter = 0;
    while program_counter < program.len() {
        let line = program[program_counter];
        let instruction = parsed_program.entry(program_counter).or_insert_with(|| {
            match interpret(line) {
                Ok(instr) => instr,
                Err(msg) => panic!(format!("error {} at {}: {}", msg, program_counter, line)),
            }
        });
        println!("{:?}", instruction);
        program_counter += 1;
    }
    registers
}

fn interpret(line: &str) -> ParseResult<Instr> {
    let s = String::from(line);
    let mut itr = s.split_ascii_whitespace();
    match itr.next() {
        Some("mov") => Ok(Instr::Mov(parse_value(itr.next())?, parse_value(itr.next())?)),
        Some("inc") => Ok(Instr::Inc(parse_value(itr.next())?)),
        Some("dec") => Ok(Instr::Dec(parse_value(itr.next())?)),
        Some("jnz") => Ok(Instr::Jnz(parse_value(itr.next())?)),
        Some(x) => Err({
            let mut msg = String::from(x);
            msg.push_str(" not recognized as instruction");
            msg
        }),
        None => Err(String::from("Empty line!")),
    }
}

fn parse_value(v_str: Option<&str>) -> ParseResult<Value> {
    if let Some(s) = v_str {
        Ok(match s.parse::<i64>() {
            Ok(n) => Value::Num(n),
            Err(_) => Value::Reg(String::from(s)),
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
}
