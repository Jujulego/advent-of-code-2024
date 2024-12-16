use std::cmp::Reverse;
use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::OwoColorize;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
            &Direction::North => vector![0, -1],
            &Direction::East => vector![1, 0],
            &Direction::South => vector![0, 1],
            &Direction::West => vector![-1, 0],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Reindeer {
    pos: Point2<i32>,
    dir: Direction,
    score: i32,
}

impl Eq for Reindeer {}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
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

    let mut mins = HashMap::new();
    let mut heap = BinaryHeap::from([
        Reverse(Reindeer { pos: start, dir: Direction::East, score: 0 })
    ]);

    while let Some(Reverse(reindeer)) = heap.pop() {
        match mins.get(&reindeer.pos) {
            Some(v) if *v <= reindeer.score => continue,
            _ => {
                mins.insert(reindeer.pos, reindeer.score);
            }
        }

        if reindeer.pos == end {
            continue;
        }

        let nexts = [
            Reindeer { pos: reindeer.pos + reindeer.dir.as_vector(), dir: reindeer.dir, score: reindeer.score + 1 },
            Reindeer { pos: reindeer.pos + reindeer.dir.turn_left().as_vector(), dir: reindeer.dir.turn_left(), score: reindeer.score + 1001 },
            Reindeer { pos: reindeer.pos + reindeer.dir.turn_right().as_vector(), dir: reindeer.dir.turn_right(), score: reindeer.score + 1001 },
        ];

        for next in nexts {
            if walls.contains(&next.pos) {
                continue;
            }

            heap.push(Reverse(next));
        }
    }

    println!("part 01: {} ({:.2?})", mins.get(&end).unwrap(), now.elapsed());
}
