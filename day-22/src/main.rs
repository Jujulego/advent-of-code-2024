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