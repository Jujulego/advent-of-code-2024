use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn shsb_random(seed: u64) -> u64 {
    let seed = ((seed << 6) ^ seed) % 16_777_216;
    let seed = ((seed >> 5) ^ seed) % 16_777_216;
    let seed = ((seed << 11) ^ seed) % 16_777_216;
    
    seed
}

fn main() {
    let secrets = read_lines!("day-22/input.txt")
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    
    // Part 01
    let now = Instant::now();
    let part01 = secrets.iter()
        .map(|secret| (0..2000).fold(*secret, |seed, _| shsb_random(seed)))
        .sum::<u64>();
    
    println!("part01: {} ({:.2?})", part01, now.elapsed());

    // Part 02
    let now = Instant::now();
    let mut sequences = Vec::with_capacity(4);

    for secret in &secrets {
        let mut seed = *secret;
        let mut previous = seed % 10;
        let mut diffs = [0; 4];
        let mut map = HashMap::new();

        for i in 0..2000 {
            seed = shsb_random(seed);

            let val = seed % 10;
            diffs.rotate_right(1);
            diffs[0] = ((val as i64) - (previous as i64)) as i8;

            if i >= 4 {
                map.entry(diffs).or_insert(val);
            }

            previous = val;
        }

        sequences.push(map);
    }

    let mut marks = HashSet::new();
    let mut part02 = 0;

    for (idx, map) in sequences.iter().enumerate() {
        for (key, val) in map {
            if !marks.insert(key) {
                continue;
            }

            let sum = val + sequences[idx + 1..].iter()
                .filter_map(|m| m.get(key))
                .sum::<u64>();

            part02 = max(sum, part02);
        }
    }

    println!("part02: {} ({:.2?})", part02, now.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::shsb_random;

    #[test]
    fn test_shsb_random() {
        assert_eq!(shsb_random(123), 15887950);
        assert_eq!(shsb_random(15887950), 16495136);
        assert_eq!(shsb_random(16495136), 527345);
    }
}