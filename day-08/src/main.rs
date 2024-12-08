use nalgebra::{point, Point2};
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

    for antennas in antennas.values() {
        for a in antennas {
            for b in antennas {
                if a == b {
                    continue;
                }

                let v = b - a;
                let antinode1 = a - v;
                let antinode2 = b + v;

                if is_inside(&map, &antinode1) {
                    antinodes.insert(antinode1);
                }

                if is_inside(&map, &antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    // Print map with antinodes
    for (y, row) in map.iter().enumerate() {
        for (x, mut c) in row.iter().enumerate() {
            let pt = point![x as i32, y as i32];
            
            let style = if antinodes.contains(&pt) {
                if c == &'.' {
                    c = &'#';
                }
                
                Style::new().color(AnsiColors::Yellow)
            } else {
                Style::new()
            };
            
            
            print!("{}", c.style(style));
        }
        
        println!();
    }

    println!("part 01: {}", antinodes.len());
}
