use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}


#[derive(Debug)]
struct Registry {
    a: i64,
    b: i64,
    c: i64,
}

fn literal_operand(value: i64) -> i64 {
    value
}

fn combo_operand(value: i64, reg: &Registry) -> i64 {
    match value {
        0..=3 => value,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => unreachable!()
    }
}

#[repr(u8)]
#[derive(Debug)]
enum Instruction {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl Instruction {
    fn from_i64(value: &i64) -> Instruction {
        match value {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => unreachable!()
        }
    }

    fn apply(&self, operand: i64, pointer: &mut usize, reg: &mut Registry) -> Option<i64> {
        *pointer += 2;

        match self {
            Instruction::ADV => reg.a >>= combo_operand(operand, reg),
            Instruction::BXL => reg.b ^= literal_operand(operand),
            Instruction::BST => reg.b = combo_operand(operand, reg) % 8,
            Instruction::JNZ => if reg.a > 0 { *pointer = literal_operand(operand) as usize },
            Instruction::BXC => reg.b ^= reg.c,
            Instruction::OUT => return Some(combo_operand(operand, reg) % 8),
            Instruction::BDV => reg.b = reg.a >> combo_operand(operand, reg),
            Instruction::CDV => reg.c = reg.a >> combo_operand(operand, reg),
        }

        None
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::ADV => write!(f, "adv"),
            Instruction::BXL => write!(f, "bxl"),
            Instruction::BST => write!(f, "bst"),
            Instruction::JNZ => write!(f, "jnz"),
            Instruction::BXC => write!(f, "bxc"),
            Instruction::OUT => write!(f, "out"),
            Instruction::BDV => write!(f, "bdv"),
            Instruction::CDV => write!(f, "cdv"),
        }
    }
}

fn run(program: &[i64], mut reg: Registry) -> Vec<i64> {
    let mut pointer = 0;
    let mut output = Vec::new();

    while let Some(instruction) = program.get(pointer).map(Instruction::from_i64) {
        if let Some(out) = instruction.apply(program[pointer + 1], &mut pointer, &mut reg) {
            output.push(out);
        }
    }

    output
}

fn main() {
    let mut lines  = read_lines!("day-17/input.txt");

    // Load registry
    let a = lines.next().unwrap()[12..].parse::<i64>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<i64>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<i64>().unwrap();
    let reg = Registry { a, b, c };

    // Load program
    assert!(lines.next().unwrap().is_empty());
    let program = lines.next().unwrap()[9..]
        .split(',')
        .map(|p| p.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // Part 01 !
    let now = Instant::now();
    let output = run(&program, reg);

    println!("part 01: {} ({:.2?})", output.iter().join(","), now.elapsed());

    // Part 02 !
    let now = Instant::now();
    let mut stack = VecDeque::from([0]);
    let mut results = Vec::new();

    while let Some(base) = stack.pop_front() {
        for n in [0o0, 0o1, 0o2, 0o3, 0o4, 0o5, 0o6, 0o7] {
            let a = (base << 3) | n;
            let output = run(&program, Registry { a, b: 0, c: 0 });

            if program.ends_with(&output) {
                if output.len() == program.len() {
                    // println!("full match !    {a:#o}");
                    results.push(a);
                } else if a != 0 {
                    // println!("partial match ! {a:#o}");
                    stack.push_front(a);
                }
            }
        }
    }

    println!("part 02: ({:.2?})", now.elapsed());
    results.sort();

    for a in results {
        let output = run(&program, Registry { a, b: 0, c: 0 });
        println!("- {a} => {}", output.iter().join(","));
    }
}
