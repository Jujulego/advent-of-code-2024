use std::collections::HashSet;
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

const UP: Vector2<i32> = vector![0, -1];
const RIGHT: Vector2<i32> = vector![1, 0];
const DOWN: Vector2<i32> = vector![0, 1];
const LEFT: Vector2<i32> = vector![-1, 0];

fn turn_right(dir: Vector2<i32>) -> Vector2<i32> {
    if dir == UP {
        RIGHT
    } else if dir == RIGHT {
        DOWN
    } else if dir == DOWN {
        LEFT
    } else if dir == LEFT {
        UP
    } else {
        unreachable!()
    }
}

fn is_inside(map: &[Vec<char>], point: &Point2<i32>) -> bool {
    (0..map.len() as i32).contains(&point.y) && (0..map[0].len() as i32).contains(&point.x)
}

fn look_at<'a>(map: &'a [Vec<char>], point: &Point2<i32>) -> Option<&'a char> {
    map.get(point.y as usize).and_then(|row| row.get(point.x as usize))
}

fn main() {
    // Load map
    let map: Vec<Vec<char>> = read_lines!("day-06/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    // Search start
    let mut position = point![0, 0];

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                position = point![x as i32, y as i32];
            }
        }
    }

    // Run !
    let mut direction = UP;
    let mut visited = HashSet::new();

    while is_inside(&map, &position) {
        let next = position + direction;
        visited.insert(position);

        if look_at(&map, &next) == Some(&'#') {
            direction = turn_right(direction);
        } else {
            position = next;
        }
    }

    println!("part 01: {}", visited.len());
}
