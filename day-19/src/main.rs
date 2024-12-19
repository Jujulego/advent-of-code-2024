use std::collections::{HashMap, HashSet, VecDeque};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn is_possible(design: &str, patterns: &HashMap<char, Vec<String>>) -> bool {
    let mut stack = VecDeque::from([design]);
    let mut marks = HashSet::new();

    while let Some(left) = stack.pop_front() {
        println!("left \x1b[90m{}\x1b[m{left}\x1b[1A", &design[..design.len() - left.len()]);

        if !marks.insert(left) {
            continue;
        }

        if let Some(first) = left.chars().next() {
            for pattern in patterns.get(&first).unwrap_or(&Vec::new()) {
                if left.starts_with(pattern) {
                    stack.push_front(&left[pattern.len()..]);
                }
            }
        } else {
            println!("\x1b[32mok\x1b[m   {design}");
            return true;
        }
    }

    println!("\x1b[31mfail\x1b[m {design}");
    false
}

fn main() {
    // Parse input
    let mut lines = read_lines!("day-19/input.txt");

    let _patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    let mut patterns = HashMap::new();

    for pattern in _patterns {
        patterns
            .entry(pattern.chars().next().unwrap())
            .or_insert_with(Vec::new)
            .push(pattern);
    }

    assert_eq!(lines.next().as_deref(), Some(""));
    let designs = lines.collect::<Vec<_>>();

    // Part 1
    let part01 = designs
        .iter()
        .filter(|design| is_possible(design, &patterns))
        .count();

    println!("part01: {}", part01);
}
