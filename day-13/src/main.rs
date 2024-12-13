use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Claw {
    pos: Point2<i32>,
    a_cnt: u8,
    b_cnt: u8,
}

impl Claw {
    fn cost(&self) -> u32 {
        self.a_cnt as u32 * 3 + self.b_cnt as u32
    }
}

impl Ord for Claw {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for Claw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn parse_button_line(line: String) -> Vector2<i32> {
    let mut parts = line.split_whitespace();
    assert_eq!(parts.next().unwrap(), "Button");
    parts.next().unwrap(); // "A:"

    let x = parts.next().unwrap(); // X+{n},
    let x = x[2..x.len() - 1].parse::<i32>().unwrap();

    let y = parts.next().unwrap(); // Y+{n}
    let y = y[2..].parse::<i32>().unwrap();

    vector![x, y]
}

fn parse_prize_line(line: String) -> Point2<i32> {
    let mut parts = line.split_whitespace();
    assert_eq!(parts.next().unwrap(), "Prize:");

    let x = parts.next().unwrap(); // X={n},
    let x = x[2..x.len() - 1].parse::<i32>().unwrap();

    let y = parts.next().unwrap(); // Y={n}
    let y = y[2..].parse::<i32>().unwrap();

    point![x, y]
}

fn search_path(a_button: Vector2<i32>, b_button: Vector2<i32>, prize: Point2<i32>) -> Option<u32> {
    let mut marks = HashSet::new();
    let mut heap = BinaryHeap::from([
        Reverse(Claw {
            pos: point![0, 0],
            a_cnt: 0,
            b_cnt: 0,
        })
    ]);

    while let Some(Reverse(claw)) = heap.pop() {
        if claw.pos == prize {
            return Some(claw.cost());
        }
        
        if claw.pos.x > prize.x || claw.pos.y > prize.y {
            continue;
        }
        
        if marks.contains(&claw) {
            continue;
        }
        
        marks.insert(claw);

        if claw.a_cnt < 100 {
            heap.push(Reverse(Claw {
                pos: claw.pos + a_button,
                a_cnt: claw.a_cnt + 1,
                b_cnt: claw.b_cnt,
            }));
        }

        if claw.b_cnt < 100 {
            heap.push(Reverse(Claw {
                pos: claw.pos + b_button,
                a_cnt: claw.a_cnt,
                b_cnt: claw.b_cnt + 1,
            }));
        }
    }

    None
}

fn main() {
    let mut lines = read_lines!("day-13/input.txt");
    let mut part01 = 0;

    loop {
        let a_button = parse_button_line(lines.next().unwrap());
        let b_button = parse_button_line(lines.next().unwrap());
        let prize = parse_prize_line(lines.next().unwrap());
        
        if let Some(cost) = search_path(a_button, b_button, prize) {
            part01 += cost;
        }

        if lines.next().is_none() {
            break;
        }
    }
    
    println!("part 01: {}", part01);
}
