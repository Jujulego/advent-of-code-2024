use std::fmt::Display;
use std::time::Instant;
use itertools::Itertools;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}


#[derive(Debug)]
struct Registry {
    a: i32,
    b: i32,
    c: i32,
}

fn literal_operand(value: u8) -> i32 {
    value as i32
}

fn combo_operand(value: u8, reg: &Registry) -> i32 {
    match value {
        0..=3 => value as i32,
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
    fn from_u8(value: &u8) -> Instruction {
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

    fn apply(&self, operand: u8, pointer: &mut usize, reg: &mut Registry) -> Option<i32> {
        *pointer += 2;

        match self {
            Instruction::ADV => reg.a /= 1 << combo_operand(operand, reg),
            Instruction::BXL => reg.b ^= literal_operand(operand),
            Instruction::BST => reg.b = combo_operand(operand, reg) % 8,
            Instruction::JNZ => if reg.a > 0 { *pointer = literal_operand(operand) as usize },
            Instruction::BXC => reg.b ^= reg.c,
            Instruction::OUT => return Some(combo_operand(operand, reg) % 8),
            Instruction::BDV => reg.b = reg.a / (1 << combo_operand(operand, reg)),
            Instruction::CDV => reg.c = reg.a / (1 << combo_operand(operand, reg)),
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

fn main() {
    let mut lines  = read_lines!("day-17/input.txt");

    // Load registry
    let a = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<i32>().unwrap();
    let mut reg = Registry { a, b, c };

    // Load program
    assert!(lines.next().unwrap().is_empty());
    let program = lines.next().unwrap()[9..]
        .split(',')
        .map(|p| p.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // Run !
    let now = Instant::now();
    let mut pointer = 0;
    let mut output = Vec::new();

    while let Some(instruction) = program.get(pointer).map(Instruction::from_u8) {
        // print!("{}({})", instruction, program[pointer + 1]);
        if let Some(out) = instruction.apply(program[pointer + 1], &mut pointer, &mut reg) {
            output.push(out);
        }
        // println!(" => {:?}", reg);
    }

    println!("part 01: {} ({:.2?})", output.iter().join(","), now.elapsed());
}
