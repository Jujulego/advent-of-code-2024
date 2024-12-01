use std::collections::HashMap;
use std::iter::zip;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn main() {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let mut right_count = HashMap::new();

    for line in read_lines!("day-01/input.txt") {
        let locations: Vec<u32> = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        left_list.push(locations[0]);
        right_list.push(locations[1]);

        *right_count.entry(locations[1]).or_insert(0) += 1;
    }

    left_list.sort();
    right_list.sort();

    let sum: u32 = zip(&left_list, &right_list)
        .map(|(l, &r)| l.abs_diff(r))
        .sum();

    let similarity: u32 = left_list.iter()
        .map(|l| l * right_count.get(l).unwrap_or(&0))
        .sum();

    println!("part 01: {}", sum);
    println!("part 02: {}", similarity);
}
