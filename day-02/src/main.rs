
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
    let mut safe_cnt = 0;

    for line in read_lines!("day-02/input.txt") {
        let spans = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();

        let same_sign = spans[1..].iter()
            .all(|n| n * spans[0] > 0);

        let below_3 = spans.iter().all(|n| n.abs() <= 3);

        if same_sign && below_3 {
            safe_cnt += 1;
        }
    }

    println!("part 01: {}", safe_cnt);
}
