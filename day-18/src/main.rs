use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;
use std::time::Instant;
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const BITS_COUNT: usize = 1024;
const MEMORY_X_LIMITS: RangeInclusive<i32> = 0..=70;
const MEMORY_Y_LIMITS: RangeInclusive<i32> = 0..=70;
const END: Point2<i32> = point![70, 70];

const STEPS: [Vector2<i32>; 4] = [
    vector![ 1,  0],
    vector![ 0,  1],
    vector![-1,  0],
    vector![ 0, -1],
];

fn main() {
    // Load bits
    let bits = read_lines!("day-18/input.txt")
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i32>().unwrap());
            point![parts.next().unwrap(), parts.next().unwrap()]
        })
        .take(BITS_COUNT)
        .collect::<HashSet<_>>();

    // Part 01
    let now = Instant::now();
    let mut queue = VecDeque::from([point![0, 0]]);
    let mut marks = HashSet::new();
    let mut distances = HashMap::from([(point![0, 0], 0)]);

    'dfs: while let Some(point) = queue.pop_front() {
        if marks.contains(&point) {
            continue;
        }

        marks.insert(point);
        let current = *distances.get(&point).unwrap();

        for step in &STEPS {
            let next = point + step;
            
            if bits.contains(&next) {
                continue;
            }
            
            distances.insert(next, current + 1);

            if next == END {
                break 'dfs;
            }

            if MEMORY_X_LIMITS.contains(&next.x) && MEMORY_Y_LIMITS.contains(&next.y) {
                queue.push_back(next);
            }
        }
    }

    println!("part 01: {} ({:.2?})", distances.get(&END).unwrap(), now.elapsed());
}
