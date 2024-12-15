use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Element {
    Wall,
    Box,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Element::Wall => write!(f, "#"),
            Element::Box => write!(f, "O")
        }
    }
}

fn push(map: &mut HashMap<Point2<i32>, Element>, from: Point2<i32>, dir: Vector2<i32>) -> Point2<i32> {
    let mut target = from + dir;

    loop {
        match map.get(&target) {
            Some(Element::Wall) => break from,
            Some(Element::Box) => {
                target += dir;
            }
            None => {
                let next = from + dir;

                map.insert(target, Element::Box);
                map.remove(&next);

                break next;
            }
        }
    }
}

fn print_map(map: &HashMap<Point2<i32>, Element>, robot: &Point2<i32>) {
    let max_x = map.keys().map(|p| p.x).max().unwrap();
    let max_y = map.keys().map(|p| p.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pt = point![x, y];

            if pt == *robot {
                print!("{}", "@".yellow().bold());
            } else if let Some(element) = map.get(&pt) {
                print!("{}", element);
            } else {
                print!("{}", ".".bright_black());
            }
        }

        println!();
    }
}

fn main() {
    // Parse input
    let mut map = HashMap::new();
    let mut robot = None;
    let mut moves = Vec::new();

    for (y, line) in read_lines!("day-15/input.txt").enumerate() {
        if line.starts_with("#") {
            for (x, c) in line.chars().enumerate() {
                let pt = point![x as i32, y as i32];

                match c {
                    '#' => { map.insert(pt, Element::Wall); },
                    'O' => { map.insert(pt, Element::Box); },
                    '@' => { robot = Some(pt); },
                    '.' => {},
                    _ => unreachable!(),
                }
            }
        } else if !line.is_empty() {
            moves.extend(
                line.chars()
                    .map(|c| match c {
                        '<' => vector![-1, 0],
                        '^' => vector![0, -1],
                        '>' => vector![1, 0],
                        'v' => vector![0, 1],
                        _ => unreachable!(),
                    })
            );
        }
    }

    // Part 01
    let now = Instant::now();
    let mut position = robot.unwrap();

    for mov in moves {
        position = push(&mut map, position, mov);
    }

    print_map(&map, &position);

    let part01 =  map.keys()
        .filter(|pt| map.get(pt).is_some_and(|&e| e == Element::Box))
        .map(|pt| pt.y * 100 + pt.x)
        .sum::<i32>();

    println!("part01: {} ({:.2?})", part01, now.elapsed());
}
