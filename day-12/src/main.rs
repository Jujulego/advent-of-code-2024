use std::collections::{HashSet, VecDeque};
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}


const DIRECTIONS: [Vector2<i32>; 4] = [
    vector![1, 0],
    vector![0, 1],
    vector![-1, 0],
    vector![0, -1],
];

fn get_plant<'a>(map: &'a [Vec<char>], point: &Point2<i32>) -> Option<&'a char> {
    map.get(point.y as usize)
        .and_then(|row| row.get(point.x as usize))
}

fn perimeter(area: &HashSet<Point2<i32>>) -> usize {
    area.iter()
        .map(|pt| DIRECTIONS.iter()
            .map(|dir| pt + dir)
            .filter(|nx| !area.contains(nx))
            .count()
        )
        .sum::<usize>()
}

fn main() {
    let map = read_lines!("day-12/input.txt")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut area_stack = VecDeque::from([point![0, 0]]);
    let mut marks = HashSet::new();
    let mut part01 = 0;

    while let Some(start) = area_stack.pop_front() {
        let area_plant = get_plant(&map, &start).unwrap();

        let mut stack = VecDeque::from([start]);
        let mut area = HashSet::new();

        while let Some(point) = stack.pop_front() {
            // Mark
            if marks.contains(&point) {
                continue;
            }

            area.insert(point);
            marks.insert(point);

            // Look around
            for dir in &DIRECTIONS {
                let next = point + dir;

                // Check area
                match get_plant(&map, &next) {
                    Some(next_plant) if next_plant == area_plant => {
                        stack.push_front(next);
                    }
                    Some(_) => {
                        area_stack.push_front(next);
                    }
                    None => {}
                }
            }
        }

        if !area.is_empty() {
            part01 += area.len() * perimeter(&area);
        }
    }

    println!("part 01: {part01}");
}
