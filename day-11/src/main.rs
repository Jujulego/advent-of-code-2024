use std::collections::HashMap;
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn rule(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let len = stone.ilog10() + 1;

    if len % 2 == 0 {
        let factor = 10u64.pow(len / 2);
        vec![stone / factor, stone % factor]
    } else {
        vec![stone * 2024]
    }
}

fn blink(cnt: u8, stone: u64, cache: &mut HashMap<(u8, u64), u64>) -> u64 {
    if cnt == 0 {
        1
    } else if let Some(res) = cache.get(&(cnt, stone)) {
        *res
    } else {
        let res = rule(stone).iter()
            .map(|s| blink(cnt - 1, *s, cache))
            .sum();

        cache.insert((cnt, stone), res);

        res
    }
}

fn main() {
    let mut cache = HashMap::new();
    let stones = read_lines!("day-11/input.txt")
        .next().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part 01
    let now = Instant::now();
    let part01 = stones.iter()
        .map(|&s| blink(25, s, &mut cache))
        .sum::<u64>();

    println!("part 01: {part01} ({:.2?})", now.elapsed());

    // Part 02
    let now = Instant::now();
    let part02 = stones.iter()
        .map(|&s| blink(75, s, &mut cache))
        .sum::<u64>();

    println!("part 02: {part02} ({:.2?})", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_0() {
        assert_eq!(rule(0), vec![1]);
    }

    #[test]
    fn rule_1() {
        assert_eq!(rule(1), vec![2024]);
    }

    #[test]
    fn rule_10() {
        assert_eq!(rule(10), vec![1, 0]);
    }

    #[test]
    fn rule_99() {
        assert_eq!(rule(99), vec![9, 9]);
    }

    #[test]
    fn rule_999() {
        assert_eq!(rule(999), vec![2021976]);
    }
}