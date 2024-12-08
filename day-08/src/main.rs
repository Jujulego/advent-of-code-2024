use nalgebra::{point, Point2, Vector2};
use owo_colors::{AnsiColors, OwoColorize, Style};
use std::collections::{HashMap, HashSet};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn is_inside(map: &Vec<Vec<char>>, point: &Point2<i32>) -> bool {
    map.get(point.y as usize)
        .and_then(|row| row.get(point.x as usize))
        .is_some()
}

fn search_antinodes(map: &Vec<Vec<char>>, mut first: Point2<i32>, vec: &Vector2<i32>) -> Vec<Point2<i32>> {
    let mut result = Vec::new();

    while is_inside(map, &first) {
        result.push(first);
        first += vec;
    }

    result
}

fn main() {
    // Load map
    let map: Vec<Vec<char>> = read_lines!("day-08/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    // Search antennas
    let mut antennas = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate().filter(|&(_, &c)| c != '.') {
            antennas
                .entry(c)
                .or_insert_with(HashSet::new)
                .insert(point![x as i32, y as i32]);
        }
    }

    // Search antinodes
    let mut antinodes = HashSet::new();
    let mut harmonic_antinodes = HashSet::new();

    for antennas in antennas.values() {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                let v = b - a;

                for (idx, &antinode) in search_antinodes(&map, *a, &(-v)).iter().enumerate() {
                    if idx == 1 {
                        antinodes.insert(antinode);
                    } else {
                        harmonic_antinodes.insert(antinode);
                    }
                }

                for (idx, &antinode) in search_antinodes(&map, *b, &v).iter().enumerate() {
                    if idx == 1 {
                        antinodes.insert(antinode);
                        harmonic_antinodes.insert(antinode);
                    } else {
                        harmonic_antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    // Print map with antinodes
    for (y, row) in map.iter().enumerate() {
        for (x, mut c) in row.iter().enumerate() {
            let pt = point![x as i32, y as i32];

            let mut style = Style::new();

            if antinodes.contains(&pt) {
                if c == &'.' {
                    c = &'#';
                }

                style = style.color(AnsiColors::Yellow);
            } else if harmonic_antinodes.contains(&pt) {
                if c == &'.' {
                    c = &'#';
                }

                style = style.color(AnsiColors::Blue);
            }


            print!("{}", c.style(style));
        }

        println!();
    }

    println!("part 01: {}", antinodes.len());
    println!("part 02: {}", harmonic_antinodes.len());
}
