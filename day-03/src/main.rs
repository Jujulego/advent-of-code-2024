use regex::Regex;

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
    let re = Regex::new(r"mul\((?<a>[0-9]+),(?<b>[0-9]+)\)").unwrap();
    let mut sum = 0;

    for line in read_lines!("day-03/input.txt") {
        for op in re.captures_iter(&line) {
            let a: i32 = op.name("a").unwrap().as_str().parse().unwrap();
            let b: i32 = op.name("b").unwrap().as_str().parse().unwrap();

            sum += a * b;
        }
    }

    println!("part 01: {sum}");
}
