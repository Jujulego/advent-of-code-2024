use nalgebra::{point, vector, Point2, Vector2};
use std::collections::{HashSet, VecDeque};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const STEPS: [Vector2<i32>; 4] = [
    vector![ 1,  0],
    vector![ 0,  1],
    vector![-1,  0],
    vector![ 0, -1],
];

fn get_height<'a>(map: &'a [Vec<u8>], point: &Point2<i32>) -> Option<&'a u8> {
    map.get(point.y as usize)?.get(point.x as usize)
}

fn reachable_pics(map: &[Vec<u8>], start: &Point2<i32>) -> (usize, usize) {
    let mut stack = VecDeque::from([*start]);
    let mut results = HashSet::new();
    let mut trails = 0;

    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();
        let height = get_height(map, &current).unwrap();

        if height == &9 {
            trails += 1;
            results.insert(current);
        } else {
            for step in &STEPS {
                let next = current + step;

                if let Some(next_height) = get_height(map, &next) {
                    if *next_height == height + 1 {
                        stack.push_front(next);
                    }
                }
            }
        }
    }

    (results.len(), trails)
}

fn main() {
    // Load map
    let mut map = Vec::new();
    let mut starts = HashSet::new();

    for (y, line) in read_lines!("day-10/input.txt").enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            row.push(c as u8 - b'0');

            if c == '0' {
                starts.insert(point![x as i32, y as i32]);
            }
        }

        map.push(row);
    }

    // Part 1
    let mut part01 = 0;
    let mut part02 = 0;
    
    for start in starts {
        let (a, b) = reachable_pics(&map, &start);
        
        part01 += a;
        part02 += b;
    }
    
    println!("part 01: {part01}");
    println!("part 02: {part02}");
}
