use std::collections::{HashMap, HashSet};
use std::mem::swap;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn check_update(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Option<(usize, usize)> {
    let mut previous = HashSet::new();

    for (idx, page) in update.iter().enumerate() {
        if let Some(next) = rules.get(page) {
            let common = previous.intersection(next)
                .map(|p| update[..idx].iter().position(|r| r == p).unwrap())
                .min();

            if let Some(index) = common {
                return Some((idx, index)); // <= returns index pair to invert
            }
        }

        previous.insert(*page);
    }

    None
}

fn main() {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut rules_loaded = false;
    let mut part01 = 0;
    let mut part02 = 0;
    let mut moves = 0;

    for line in read_lines!("day-05/input.txt") {
        if line.is_empty() {
            rules_loaded = true;
            continue;
        }

        if !rules_loaded {
            // Load rules
            let mut parts = line.split('|')
                .map(|p| p.parse::<i32>().unwrap());

            let x = parts.next().unwrap();
            let y = parts.next().unwrap();

            rules.entry(x)
                .or_default()
                .insert(y);
        } else {
            // Parse updates
            let mut update: Vec<i32> = line.split(',').map(|p| p.parse().unwrap()).collect();
            let mut correct = true;

            while let Some((to_move_idx, before_idx)) = check_update(&update, &rules) {
                assert!(to_move_idx > before_idx);
                correct = false;

                let mut tmp = update[to_move_idx];

                for page in &mut update[before_idx..=to_move_idx] {
                    swap(page, &mut tmp);
                }

                moves += 1;
            }

            if correct {
                part01 += update[update.len() / 2];
            } else {
                part02 += update[update.len() / 2];
            }
        }
    }

    println!("part 01: {part01}");
    println!("part 02: {part02} ({moves})");
}
