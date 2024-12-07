use std::collections::VecDeque;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn is_calibrated(terms: &[u64], result: u64) -> bool {
    let mut queue = VecDeque::from([(terms[0], 1)]);

    while !queue.is_empty() {
        let (val, idx) = queue.pop_front().unwrap();

        if let Some(term) = terms.get(idx) {
            for res in [val + term, val * term] {
                if res > result {
                    continue;
                }
                
                queue.push_back((res, idx + 1));
            }
        } else if val == result { // Found !
            return true;
        }
    }

    false
}

fn main() {
    let mut part01 = 0;

    for line in read_lines!("day-07/input.txt") {
        let colon_idx = line.find(':').unwrap();
        let result = line[..colon_idx].parse::<u64>().unwrap();
        let terms = line[colon_idx + 2..]
            .split_whitespace()
            .map(|term| term.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        
        if is_calibrated(&terms, result) {
            part01 += result;
        }
    }
    
    println!("part 01: {part01}");
}
