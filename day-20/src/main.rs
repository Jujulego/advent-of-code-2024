use std::collections::{BTreeMap, HashMap, HashSet};
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const UP: Vector2<i32> = vector![0, -1];
const RIGHT: Vector2<i32> = vector![1, 0];
const DOWN: Vector2<i32> = vector![0, 1];
const LEFT: Vector2<i32> = vector![-1, 0];
const STEPS: [Vector2<i32>; 4] = [UP, RIGHT, DOWN, LEFT];

fn main() {
    // Load map
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    
    for (y, line) in read_lines!("day-20/input.txt").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pt = point![x as i32, y as i32];
            
            match c { 
                '#' => { walls.insert(pt); }
                'S' => { start = Some(pt); }
                'E' => { end = Some(pt); }
                '.' => {}
                _ => unreachable!()
            }
        }
    }
    
    let start = start.unwrap();
    let end = end.unwrap();
    
    // Compute path
    let mut picos: HashMap<Point2<i32>, u32> = HashMap::new();
    let mut pos = end;
    let mut steps = 0;
    
    loop {
        picos.insert(pos, steps);
        steps += 1;
        
        if pos == start {
            break;
        }
        
        pos = STEPS.iter()
            .map(|step| pos + step)
            .filter(|p| !walls.contains(p))
            .filter(|p| !picos.contains_key(p))
            .next().unwrap();
    }
    
    // Search tunnels
    let mut cheats = BTreeMap::new();
    
    for (pt, &cost) in &picos {
        for step in STEPS.iter() {
            let pt1 = pt + step;
            
            if !walls.contains(&pt1) {
                continue;
            }
            
            for step in STEPS.iter() {
                let pt2 = pt1 + step;

                if let Some(&next) = picos.get(&pt2) {
                    if next < cost && next.abs_diff(cost) > 2 {
                        *cheats.entry(next.abs_diff(cost) - 2).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    let part01: u32 = cheats.iter()
        .filter(|(&gain, _)| gain >= 100)
        .map(|(_, &count)| count)
        .sum();
    
    println!("part01: {}", part01);
}
