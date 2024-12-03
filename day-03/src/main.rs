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
    let re = Regex::new(r"((?<op>mul|do|don't)\(((?<a>[0-9]+),(?<b>[0-9]+))?\))").unwrap();
    let mut enabled = true;
    let mut part01 = 0;
    let mut part02 = 0;

    for line in read_lines!("day-03/input.txt") {
        for expr in re.captures_iter(&line) {
            match expr.name("op").unwrap().as_str() {
                "mul" => {
                    if let Some(a) = expr.name("a") {
                        let a = a.as_str().parse::<i32>().unwrap();
                        let b = expr.name("b").unwrap().as_str().parse::<i32>().unwrap();
                        let r = a * b;

                        part01 += r;

                        if enabled {
                            part02 += r;
                        }
                    }
                }
                "do" => {
                    enabled = true;
                }
                "don't" => {
                    enabled = false;
                }
                _ => unreachable!(),
            }
        }
    }

    println!("part 01: {part01}");
    println!("part 02: {part02}");
}
