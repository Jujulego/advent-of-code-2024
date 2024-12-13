use std::cmp::{min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

#[derive(Clone, Copy, Debug)]
struct ClawMachine {
    a_button: Vector2<u64>,
    b_button: Vector2<u64>,
    prize: Point2<u64>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ClawState {
    a_cnt: u64,
    b_cnt: u64,
}

impl ClawState {
    fn cost(&self) -> u64 {
        self.a_cnt * 3 + self.b_cnt
    }

    fn pos(&self, machine: &ClawMachine) -> Point2<u64> {
        ((self.a_cnt * machine.a_button) + (self.b_cnt * machine.b_button)).into()
    }
}

impl Ord for ClawState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for ClawState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn parse_button_line(line: String) -> Vector2<u64> {
    let mut parts = line.split_whitespace();
    assert_eq!(parts.next().unwrap(), "Button");
    parts.next().unwrap(); // "A:"

    let x = parts.next().unwrap(); // X+{n},
    let x = x[2..x.len() - 1].parse().unwrap();

    let y = parts.next().unwrap(); // Y+{n}
    let y = y[2..].parse().unwrap();

    vector![x, y]
}

fn parse_prize_line(line: String) -> Point2<u64> {
    let mut parts = line.split_whitespace();
    assert_eq!(parts.next().unwrap(), "Prize:");

    let x = parts.next().unwrap(); // X={n},
    let x = x[2..x.len() - 1].parse().unwrap();

    let y = parts.next().unwrap(); // Y={n}
    let y = y[2..].parse().unwrap();

    point![x, y]
}

fn search_path(machine: &ClawMachine) -> Option<u64> {
    let mut marks = HashSet::new();
    let mut heap = BinaryHeap::from([
        Reverse(ClawState {
            a_cnt: 0,
            b_cnt: 0,
        })
    ]);

    while let Some(Reverse(claw)) = heap.pop() {
        let pos: Point2<u64> = claw.pos(machine);

        if pos == machine.prize {
            return Some(claw.cost());
        }

        if pos.x > machine.prize.x || pos.y > machine.prize.y {
            continue;
        }

        if marks.contains(&claw) {
            continue;
        }

        marks.insert(claw);

        if claw.a_cnt < 100 {
            heap.push(Reverse(ClawState {
                a_cnt: claw.a_cnt + 1,
                b_cnt: claw.b_cnt,
            }));
        }

        if claw.b_cnt < 100 {
            heap.push(Reverse(ClawState {
                a_cnt: claw.a_cnt,
                b_cnt: claw.b_cnt + 1,
            }));
        }
    }

    None
}

fn search_path_v2(machine: &ClawMachine) -> Option<u64> {
    let mut state = ClawState {
        a_cnt: 0,
        b_cnt: min(machine.prize.x / machine.b_button.x, machine.prize.y / machine.b_button.y),
    };

    println!("{:?}", machine);
    while state.b_cnt > 0 {
        println!("{:?} => {:?}", state, state.pos(machine));

        loop {
            let pos = state.pos(machine);

            if pos.x >= machine.prize.x || pos.y >= machine.prize.y {
                break;
            } else {
                state.a_cnt += 1;
            }
        }

        let pos = state.pos(machine);

        if pos == machine.prize {
            return Some(state.cost());
        } else {
            state.b_cnt -= 1;
        }
    }

    None
}

fn search_path_v3(machine: &ClawMachine) -> Option<u64> {
    let ka = (machine.a_button.y as f64) / (machine.a_button.x as f64);
    let kb = (machine.b_button.y as f64) / (machine.b_button.x as f64);

    let ix = (-ka * (machine.prize.x as f64) + (machine.prize.y as f64)) / (kb - ka);
    let ix = ix.round() as u64;

    let a_cnt = ix.abs_diff(machine.prize.x) / machine.a_button.x;
    let b_cnt = ix / machine.b_button.x;

    let end = (a_cnt * machine.a_button) + (b_cnt * machine.b_button);
    let end: Point2<u64> = end.into();


    if end == machine.prize {
        println!("{a_cnt} + {b_cnt} => {:?} match!", end);
        Some(a_cnt * 3 + b_cnt)
    } else {
        println!("{a_cnt} + {b_cnt} => {:?} fail!", end);
        None
    }
}

fn main() {
    let mut lines = read_lines!("day-13/input.txt");
    let mut machines = Vec::new();

    loop {
        let a_button = parse_button_line(lines.next().unwrap());
        let b_button = parse_button_line(lines.next().unwrap());
        let prize = parse_prize_line(lines.next().unwrap());

        machines.push(ClawMachine { a_button, b_button, prize });

        if lines.next().is_none() {
            break;
        }
    }

    let now = Instant::now();
    let part01 = machines.iter()
        .filter_map(search_path_v3)
        .sum::<u64>();

    println!("part 01: {} ({:.2?})", part01, now.elapsed());

    let now = Instant::now();
    let part02 = machines.iter()
        .map(|&machine| ClawMachine {
            prize: point![10000000000000 + machine.prize.x, 10000000000000 + machine.prize.y],
            ..machine
        })
        .filter_map(|m| search_path_v3(&m))
        .sum::<u64>();

    println!("part 02: {} ({:.2?})", part02, now.elapsed());
}
