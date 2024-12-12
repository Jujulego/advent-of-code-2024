use nalgebra::{point, vector, Point2, Vector2};
use std::collections::{HashSet, VecDeque};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const RIGHT: Vector2<i32> = vector![1, 0];
const DOWN: Vector2<i32> = vector![0, 1];
const LEFT: Vector2<i32> = vector![-1, 0];
const UP: Vector2<i32> = vector![0, -1];

const DIRECTIONS: [Vector2<i32>; 4] = [RIGHT, DOWN, LEFT, UP];

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

fn x_sides(area: &HashSet<Point2<i32>>) -> usize {
    let mut sides = 0;
    
    let min_x = area.iter().map(|pt| pt.x).min().unwrap();
    let max_x = area.iter().map(|pt| pt.x).max().unwrap();
    
    for x in min_x..=max_x {
        let mut column = area.iter()
            .filter(|pt| pt.x == x)
            .collect::<Vec<_>>();
        
        if column.is_empty() {
            continue;
        }
        
        column.sort_by(|a, b| a.y.cmp(&b.y));

        for dir in [LEFT, RIGHT] {
            let walls = column.iter()
                .map(|&pt| pt + dir)
                .filter(|pt| !area.contains(&pt))
                .map(|pt| pt.y)
                .collect::<Vec<_>>();

            if let Some(mut prev) = walls.first() {
                sides += 1;

                for y in walls[1..].iter() {
                    if y - prev > 1 {
                        sides += 1;
                    }

                    prev = y;
                }
            }
        }
    }
    
    sides
}

fn y_sides(area: &HashSet<Point2<i32>>) -> usize {
    let mut sides = 0;

    let min_y = area.iter().map(|pt| pt.y).min().unwrap();
    let max_y = area.iter().map(|pt| pt.y).max().unwrap();

    for y in min_y..=max_y {
        let mut column = area.iter()
            .filter(|pt| pt.y == y)
            .collect::<Vec<_>>();

        if column.is_empty() {
            continue;
        }

        column.sort_by(|a, b| a.x.cmp(&b.x));

        for dir in [UP, DOWN] {
            let walls = column.iter()
                .map(|&pt| pt + dir)
                .filter(|pt| !area.contains(&pt))
                .map(|pt| pt.x)
                .collect::<Vec<_>>();

            if let Some(mut prev) = walls.first() {
                sides += 1;

                for x in walls[1..].iter() {
                    if x - prev > 1 {
                        sides += 1;
                    }

                    prev = x;
                }
            }
        }
    }

    sides
}

fn main() {
    let map = read_lines!("day-12/input.txt")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut area_stack = VecDeque::from([point![0, 0]]);
    let mut marks = HashSet::new();
    let mut part01 = 0;
    let mut part02 = 0;

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
            part02 += area.len() * (x_sides(&area) + y_sides(&area));
        }
    }

    println!("part 01: {part01}");
    println!("part 02: {part02}");
}
