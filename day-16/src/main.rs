use std::cmp::Reverse;
use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::OwoColorize;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Direction {
    North,
    #[default] East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn as_vector(&self) -> Vector2<i32> {
        match self {
            Direction::North => vector![0, -1],
            Direction::East => vector![1, 0],
            Direction::South => vector![0, 1],
            Direction::West => vector![-1, 0],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Reindeer {
    pos: Point2<i32>,
    dir: Direction,
    turns: i32,
    moves: i32,
}

impl Reindeer {
    fn score(&self) -> i32 {
        self.turns * 1000 + self.moves
    }
}

impl Eq for Reindeer {}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn print_map(walls: &HashSet<Point2<i32>>, start: &Point2<i32>, end: &Point2<i32>) {
    let max_x = walls.iter().map(|p| p.x).max().unwrap();
    let max_y = walls.iter().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = point![x, y];

            if walls.contains(&point) {
                print!("#");
            } else if point == *start {
                print!("S");
            } else if point == *end {
                print!("E");
            } else {
                print!("{}", ".".bright_black());
            }
        }

        println!();
    }
}

fn print_map_with_paths(walls: &HashSet<Point2<i32>>, paths: &HashSet<Point2<i32>>) {
    let max_x = walls.iter().map(|p| p.x).max().unwrap();
    let max_y = walls.iter().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = point![x, y];

            if walls.contains(&point) {
                print!("#");
            } else if paths.contains(&point) {
                print!("{}", "O".yellow().bold());
            } else {
                print!("{}", ".".bright_black());
            }
        }

        println!();
    }
}

fn main() {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in read_lines!("day-16/input.txt").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pt = point![x as i32, y as i32];

            match c {
                '#' => { walls.insert(pt); },
                'S' => { start = Some(pt); },
                'E' => { end = Some(pt); },
                '.' => continue,
                _ => unreachable!()
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    // Part 01
    let now = Instant::now();

    let mut mins: HashMap<Point2<i32>, Reindeer> = HashMap::new();
    let mut heap = BinaryHeap::from([
        Reverse(Reindeer { pos: start, dir: Direction::East, moves: 0, turns: 0 }),
    ]);

    while let Some(Reverse(reindeer)) = heap.pop() {
        match mins.get(&reindeer.pos) {
            Some(v) if v.score() <= reindeer.score() => continue,
            _ => {
                mins.insert(reindeer.pos, reindeer);
            }
        }

        if reindeer.pos == end {
            continue;
        }

        let nexts = [
            Reindeer { pos: reindeer.pos + reindeer.dir.as_vector(), dir: reindeer.dir, turns: reindeer.turns, moves: reindeer.moves + 1 },
            Reindeer { pos: reindeer.pos + reindeer.dir.turn_left().as_vector(), dir: reindeer.dir.turn_left(), turns: reindeer.turns + 1, moves: reindeer.moves + 1 },
            Reindeer { pos: reindeer.pos + reindeer.dir.turn_right().as_vector(), dir: reindeer.dir.turn_right(), turns: reindeer.turns + 1, moves: reindeer.moves + 1 },
        ];

        for next in nexts {
            if walls.contains(&next.pos) {
                continue;
            }

            heap.push(Reverse(next));
        }
    }

    println!("part 01: {} ({:.2?})", mins.get(&end).unwrap().score(), now.elapsed());

    // Part 02
    let mut paths = HashSet::new();
    let mut stack = VecDeque::from([end]);

    while let Some(pt) = stack.pop_back() {
        if paths.contains(&pt) {
            continue;
        }

        paths.insert(pt);

        let reindeer = mins.get(&pt).unwrap();

        for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
            let prev = pt - dir.as_vector();
            let next = pt + dir.as_vector();

            if walls.contains(&prev) {
                continue;
            }

            // println!("{:?}({reindeer:?}) => {:?}({:?})", pt, prev, mins.get(&prev));

            if let Some(p) = mins.get(&prev) {
                if p.moves + 1 == reindeer.moves && p.turns == reindeer.turns {
                    stack.push_front(prev);
                    continue;
                }

                if p.moves + 1 == reindeer.moves && p.turns + 1 == reindeer.turns {
                    stack.push_front(prev);
                    continue;
                }

                if !paths.contains(&next) {
                    continue;
                }

                if let Some(n) = mins.get(&next) {
                    if p.moves + 2 == n.moves && p.turns == n.turns {
                        stack.push_front(prev);
                        continue;
                    }
                }
            }
        }
    }

    // print_map_with_paths(&walls, &paths);
    println!("part 02: {} ({:.2?})", paths.len(), now.elapsed());
}
