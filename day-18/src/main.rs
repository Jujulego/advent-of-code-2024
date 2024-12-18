use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::DynColors::Rgb;
use owo_colors::OwoColorize;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::ops::RangeInclusive;
use std::rc::Rc;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const BITS_COUNT: usize = 1024;
const END: Point2<i32> = point![70, 70];
const MEMORY_X_LIMITS: RangeInclusive<i32> = 0..=70;
const MEMORY_Y_LIMITS: RangeInclusive<i32> = 0..=70;

const UP: Vector2<i32> = vector![0, -1];
const RIGHT: Vector2<i32> = vector![1, 0];
const DOWN: Vector2<i32> = vector![0, 1];
const LEFT: Vector2<i32> = vector![-1, 0];

const STEPS: [Vector2<i32>; 4] = [UP, RIGHT, DOWN, LEFT];

struct Node {
    point: Point2<i32>,
    cost: u32,
    previous: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn is_previous_of(&self, node: &Node) -> bool {
        node.previous.clone().is_some_and(|p| p.borrow().point == self.point)
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn print_map(nodes: &HashMap<Point2<i32>, Rc<RefCell<Node>>>) {
    for y in MEMORY_Y_LIMITS {
        for x in MEMORY_X_LIMITS {
            let pt = point![x, y];

            if let Some(node) = nodes.get(&pt) {
                let mut dirs = STEPS
                    .map(|s| pt + s)
                    .map(|p| nodes.get(&p).is_some_and(|n|
                        n.borrow().is_previous_of(&node.borrow()) || node.borrow().is_previous_of(&n.borrow())
                    ));

                if pt == END {
                    dirs[1] = true;
                }

                let cost = node.borrow().cost;
                let color = if cost == u32::MAX { Rgb(255, 0, 0) } else { Rgb(0, 255 - ((cost * 5) % 175) as u8, 0) };

                match dirs {
                    [false, false, false, false] => print!("{}", ".".color(color)),
                    [false, false, false, true] => print!("{}", "\u{2574}".color(color)),
                    [false, false, true, false] => print!("{}", "\u{2577}".color(color)),
                    [false, true, false, false] => print!("{}", "\u{2576}".color(color)),
                    [true, false, false, false] => print!("{}", "\u{2575}".color(color)),
                    [false, false, true, true] => print!("{}", "\u{256e}".color(color)),
                    [false, true, false, true] => print!("{}", "\u{2500}".color(color)),
                    [true, false, false, true] => print!("{}", "\u{256f}".color(color)),
                    [false, true, true, false] => print!("{}", "\u{256d}".color(color)),
                    [true, false, true, false] => print!("{}", "\u{2502}".color(color)),
                    [true, true, false, false] => print!("{}", "\u{2570}".color(color)),
                    [false, true, true, true] => print!("{}", "\u{252c}".color(color)),
                    [true, false, true, true] => print!("{}", "\u{2524}".color(color)),
                    [true, true, false, true] => print!("{}", "\u{2534}".color(color)),
                    [true, true, true, false] => print!("{}", "\u{251c}".color(color)),
                    [true, true, true, true] => print!("{}", "\u{253c}".color(color)),
                }
            } else {
                print!("{}", "\u{2588}".bright_black());
            }
        }

        println!();
    }
}

fn main() {
    // Initiate map
    let mut queue = VecDeque::from([END]);
    let mut nodes = HashMap::from([
        (END, Rc::new(RefCell::new(Node { point: END, cost: 0, previous: None }))),
    ]);

    while let Some(pt) = queue.pop_front() {
        let current = nodes.get(&pt).unwrap().clone();

        for step in &STEPS {
            let next = pt + step;

            if !MEMORY_X_LIMITS.contains(&next.x) || !MEMORY_Y_LIMITS.contains(&next.y) {
                continue;
            }

            if nodes.contains_key(&next) {
                continue;
            }

            queue.push_back(next);
            nodes.insert(next, Rc::new(RefCell::new(Node {
                point: next,
                cost: current.borrow().cost + 1,
                previous: Some(current.clone()),
            })));
        }
    }

    print_map(&nodes);
    println!("Map initiated => {}", nodes.get(&point![0, 0]).unwrap().borrow().cost);
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Let bits fall !
    let mut part01 = 0;

    let bits = read_lines!("day-18/input.txt")
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i32>().unwrap());
            point![parts.next().unwrap(), parts.next().unwrap()]
        });

    for (idx, bit) in bits.enumerate() {
        nodes.remove(&bit);

        // Update paths
        let mut queue = BinaryHeap::new();

        for step in &STEPS {
            let next = bit + step;

            if let Some(node) = nodes.get(&next) {
                if node.borrow().previous.clone().is_some_and(|p| p.borrow().point == bit) {
                    node.borrow_mut().cost = u32::MAX;
                    node.borrow_mut().previous = None;

                    queue.push(Reverse(node.clone()));
                }
            }
        }

        while let Some(Reverse(node)) = queue.pop() {
            let point = node.borrow().point;

            let nexts = STEPS.iter()
                .map(|s| point + s)
                .filter_map(|n| nodes.get(&n).cloned())
                .collect::<Vec<_>>();

            // Search surrounding min
            let min_node = nexts.iter()
                .filter(|n| !node.borrow().is_previous_of(&n.borrow()))
                .filter(|n| n.borrow().cost < u32::MAX)
                .min();

            if let Some(min_node) = min_node {
                node.borrow_mut().cost = min_node.borrow().cost + 1;
                node.borrow_mut().previous = Some(min_node.clone());
            } else {
                node.borrow_mut().cost = u32::MAX;
                node.borrow_mut().previous = None;
            }

            // Update surroundings
            for next in nexts {
                // Ignore node's previous
                if next.borrow().is_previous_of(&node.borrow()) {
                    continue;
                }

                // Update current next nodes
                if node.borrow().is_previous_of(&next.borrow()) {
                    if node.borrow().cost == u32::MAX {
                        next.borrow_mut().cost = u32::MAX;
                        next.borrow_mut().previous = None;
                    } else {
                        next.borrow_mut().cost = node.borrow().cost + 1;
                    }

                    queue.push(Reverse(next.clone()));
                    continue;
                }

                // Better path ?
                if node.borrow().cost < u32::MAX && node.borrow().cost + 1 < next.borrow().cost {
                    next.borrow_mut().cost = node.borrow().cost + 1;
                    next.borrow_mut().previous = Some(node.clone());

                    queue.push(Reverse(next.clone()));
                }
            }
        }

        // Debug
        if idx % 10 == 0 {
            print!("\x1b[{}A", MEMORY_Y_LIMITS.end() + 2);
            print_map(&nodes);
            println!("\x1b[KBit #{idx} ({},{}) corrupted !", bit.x, bit.y);
            std::thread::sleep(std::time::Duration::from_millis(250));
        }

        if idx == BITS_COUNT {
            part01 = nodes.get(&point![0, 0]).unwrap().borrow().cost;
        }

        if nodes.get(&point![0, 0]).is_some_and(|n| n.borrow().previous.is_none()) {
            print!("\x1b[{}A", MEMORY_Y_LIMITS.end() + 2);
            print_map(&nodes);
            println!("\x1b[KBit #{idx} ({},{}) corrupted !", bit.x, bit.y);
            break;
        }
    }

    println!("part01: {part01}");
}
