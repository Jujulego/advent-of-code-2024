use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

#[derive(Clone, Copy, Debug)]
enum LogicOperator {
    And,
    Or,
    Xor
}

impl LogicOperator {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            LogicOperator::And => lhs & rhs,
            LogicOperator::Or => lhs | rhs,
            LogicOperator::Xor => lhs ^ rhs,
        }
    }
}

impl FromStr for LogicOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(LogicOperator::And),
            "OR" => Ok(LogicOperator::Or),
            "XOR" => Ok(LogicOperator::Xor),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug)]
struct LogicGate {
    operator: LogicOperator,
    inputs: [String; 2],
    output: String,
}

fn main() {
    // Parse input
    let mut wires = HashMap::new();
    let mut gates = Vec::new();

    let mut on_wires = true;

    for line in read_lines!("day-24/input.txt") {
        if line.is_empty() {
            on_wires = false;
            continue;
        }

        if on_wires {
            let mut parts = line.split_whitespace();

            let name = parts.next().unwrap();
            let value = parts.next().unwrap() == "1";

            wires.insert(name[..3].to_string(), Some(value));
        } else {
            let mut parts = line.split_whitespace();

            let lhs = parts.next().unwrap().to_string();
            let ope = parts.next().unwrap().parse::<LogicOperator>().unwrap();
            let rhs = parts.next().unwrap().to_string();
            assert_eq!(parts.next().unwrap(), "->");
            let out = parts.next().unwrap().to_string();

            gates.push(LogicGate {
                operator: ope,
                inputs: [lhs.clone(), rhs.clone()],
                output: out.clone(),
            });

            if !wires.contains_key(&lhs) { wires.insert(lhs, None); }
            if !wires.contains_key(&rhs) { wires.insert(rhs, None); }
            if !wires.contains_key(&out) { wires.insert(out, None); }
        }
    }
    
    // Part 01
    let now = Instant::now();
    
    let mut stack = VecDeque::from_iter(gates.iter().cloned());
    
    while let Some(gate) = stack.pop_front() {
        // Already computed
        if wires.get(&gate.output).unwrap().is_some() {
            continue;
        }
        
        let inputs = gate.inputs.map(|wire| wires.get(&wire).unwrap());
        
        if inputs.iter().all(|opt| opt.is_some()) {
            let [lhs, rhs] = inputs.map(|opt| opt.unwrap());
            wires.insert(gate.output.clone(), Some(gate.operator.apply(lhs, rhs)));
            stack.extend(gates.iter().filter(|g| g.inputs.contains(&gate.output)).cloned());
        }
    }
    
    let mut part01: u64 = 0;
    
    for (wire, _) in wires.iter().filter(|(k, v)| k.starts_with('z') && v.unwrap()) {
        let n = wire[1..].parse::<u64>().unwrap();
        part01 |= 1 << n;
    }
    
    println!("part 01: {} ({:.2?})", part01, now.elapsed());
}
