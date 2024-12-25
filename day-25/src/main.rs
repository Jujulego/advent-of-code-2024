use std::iter::zip;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn main() {
    let mut lines = read_lines!("day-25/input.txt");
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(line) = lines.next() {
        let lock = line == "#####";
        let mut values = [0; 5];

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            for (c, v) in zip(line.chars(), values.iter_mut()) {
                if c == '#' {
                    *v += 1;
                }
            }
        }

        if lock {
            locks.push(values);
        } else {
            keys.push(values.map(|v| v - 1));
        }
    }

    // Part 01
    let mut part01 = 0;
    
    for lock in &locks {
        for key in &keys {
            let fit = zip(lock, key)
                .map(|(l, k)| l + k)
                .all(|s| s <= 5);
            
            part01 += if fit { 1 } else { 0 };
        }
    }
    
    println!("part 01: {}", part01);
}
