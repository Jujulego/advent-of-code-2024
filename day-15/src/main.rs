use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::thread;
use std::time::{Duration, Instant};

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
            Element::Box => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LargeElement {
    Wall,
    LeftBox,
    RightBox,
}

impl Display for LargeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LargeElement::Wall => write!(f, "#"),
            LargeElement::LeftBox => write!(f, "["),
            LargeElement::RightBox => write!(f, "]")
        }
    }
}

fn build_large_map(map: &HashMap<Point2<i32>, Element>) -> HashMap<Point2<i32>, LargeElement> {
    let mut result = HashMap::new();

    for (pt, element) in map {
        match element {
            Element::Wall => {
                result.insert(point![pt.x * 2, pt.y], LargeElement::Wall);
                result.insert(point![pt.x * 2 + 1, pt.y], LargeElement::Wall);
            }
            Element::Box => {
                result.insert(point![pt.x * 2, pt.y], LargeElement::LeftBox);
                result.insert(point![pt.x * 2 + 1, pt.y], LargeElement::RightBox);
            }
        }
    }

    result
}

fn push(map: &mut HashMap<Point2<i32>, Element>, from: &Point2<i32>, dir: &Vector2<i32>) -> Point2<i32> {
    let mut target = from + dir;

    loop {
        match map.get(&target) {
            Some(Element::Wall) => break *from,
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

fn push_large(map: &mut HashMap<Point2<i32>, LargeElement>, from: &Point2<i32>, dir: &Vector2<i32>) -> Point2<i32> {
    let mut queue = VecDeque::from([*from]);
    let mut marks = HashSet::new();
    let mut boxes = Vec::new();

    while let Some(pt) = queue.pop_back() {
        let next = pt + dir;

        if marks.contains(&next) {
            continue;
        }

        marks.insert(next);

        match map.get(&next) {
            Some(LargeElement::Wall) => return *from,
            Some(LargeElement::LeftBox) => {
                boxes.push(next);

                queue.push_front(next);
                queue.push_front(next + vector![1, 0]);
            }
            Some(LargeElement::RightBox) => {
                boxes.push(next - vector![1, 0]);

                queue.push_front(next - vector![1, 0]);
                queue.push_front(next);
            }
            None => continue,
        }
    }

    for left_box in boxes.iter().rev() {
        let right_box = left_box + vector![1, 0];
        map.remove(left_box);
        map.remove(&right_box);

        map.insert(left_box + dir, LargeElement::LeftBox);
        map.insert(right_box + dir, LargeElement::RightBox);
    }

    from + dir
}

fn print_map<E: Display>(map: &HashMap<Point2<i32>, E>, robot: &Point2<i32>) {
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
    print!("\x1b[2J");

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

    let mut large_map = build_large_map(&map);

    // Part 01
    let now = Instant::now();
    let mut position = robot.unwrap();

    for (i, mov) in moves.iter().enumerate() {
        position = push(&mut map, &position, mov);

        // print!("\x1b[1;1H");
        // print_map(&map, &position);
        // println!("{:w$}/{}", i + 1, moves.len(), w = (moves.len().ilog10() + 1) as usize);
        //
        // thread::sleep(Duration::from_millis(50));
    }

    let part01 = map.iter()
        .filter(|(_, &el)| el == Element::Box)
        .map(|(pt, _)| pt.y * 100 + pt.x)
        .sum::<i32>();

    println!("part 01: {} ({:.2?})", part01, now.elapsed());
    println!();

    // Part 02
    let now = Instant::now();
    let mut position = robot.unwrap();
    position = point![position.x * 2, position.y];

    for (i, mov) in moves.iter().enumerate() {
        position = push_large(&mut large_map, &position, mov);

        print!("\x1b[1;1H");
        print_map(&large_map, &position);
        println!("{:w$}/{}", i + 1, moves.len(), w = (moves.len().ilog10() + 1) as usize);

        thread::sleep(Duration::from_millis(50));
    }

    let part02 = large_map.iter()
        .filter(|(_, &el)| el == LargeElement::LeftBox)
        .map(|(pt, _)| pt.y * 100 + pt.x)
        .sum::<i32>();

    println!("part 02: {} ({:.2?})", part02, now.elapsed());
}
