use itertools::Itertools;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let spans = levels.windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();

    let same_sign = spans[1..].iter()
        .all(|n| n * spans[0] > 0);

    let below_3 = spans.iter().all(|n| n.abs() <= 3);

    same_sign && below_3
}

fn main() {
    let mut safe_cnt = 0;
    let mut dampener_safe_cnt = 0;

    for line in read_lines!("day-02/input.txt") {
        let levels = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&levels) {
            safe_cnt += 1;
        }

        if levels.iter()
            .combinations(levels.len() - 1)
            .any(|levels| is_safe(&levels.iter().copied().copied().collect())) {
            dampener_safe_cnt += 1;
        }
    }

    println!("part 01: {}", safe_cnt);
    println!("part 02: {}", dampener_safe_cnt);
}
