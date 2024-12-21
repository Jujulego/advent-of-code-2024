use std::collections::HashMap;
use nalgebra::{point, vector, Point2};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

trait Keypad {
    fn start(&self) -> Point2<i32>;
    fn gap(&self) -> Point2<i32>;
    fn to_point(&self, key: &char) -> Point2<i32>;
}

struct NumericKeypad {}

impl Keypad for NumericKeypad {
    fn start(&self) -> Point2<i32> {
        point![2, 3]
    }

    fn gap(&self) -> Point2<i32> {
        point![0, 3]
    }

    fn to_point(&self, key: &char) -> Point2<i32> {
        match key {
            &'7' => point![0, 0],
            &'8' => point![1, 0],
            &'9' => point![2, 0],
            &'4' => point![0, 1],
            &'5' => point![1, 1],
            &'6' => point![2, 1],
            &'1' => point![0, 2],
            &'2' => point![1, 2],
            &'3' => point![2, 2],
            &'0' => point![1, 3],
            &'A' => point![2, 3],
            c => panic!("Unknown key: {c}"),
        }
    }
}

struct DirectionalKeypad {}

impl Keypad for DirectionalKeypad {
    fn start(&self) -> Point2<i32> {
        point![2, 0]
    }

    fn gap(&self) -> Point2<i32> {
        point![0, 0]
    }

    fn to_point(&self, key: &char) -> Point2<i32> {
        match key {
            &'^' => point![1, 0],
            &'A' => point![2, 0],
            &'<' => point![0, 1],
            &'v' => point![1, 1],
            &'>' => point![2, 1],
            c => panic!("Unknown key: {c}"),
        }
    }
}

fn build_paths(from: Point2<i32>, to: Point2<i32>) -> Vec<String> {
    let diff = to - from;

    let h = match diff.x {
        d if d < 0 => "<".to_string().repeat(d.abs() as usize),
        d if d > 0 => ">".to_string().repeat(d as usize),
        _ => "".to_string(),
    };

    let v = match diff.y {
        d if d < 0 => "^".to_string().repeat(d.abs() as usize),
        d if d > 0 => "v".to_string().repeat(d as usize),
        _ => "".to_string(),
    };

    if h.is_empty() {
        vec![v + "A"]
    } else if v.is_empty() {
        vec![h + "A"]
    } else {
        vec![h.clone() + &v + "A", v + &h + "A"]
    }
}

fn path_includes(path: &str, mut pos: Point2<i32>, gap: Point2<i32>) -> bool {
    for key in path.chars() {
        match key {
            '>' => pos += vector![1, 0],
            '<' => pos += vector![-1, 0],
            '^' => pos += vector![0, -1],
            'v' => pos += vector![0, 1],
            'A' => {}
            c => panic!("Unknown path key: {c}"),
        }

        if pos == gap {
            return true;
        }
    }

    false
}

fn code_cost(code: &str, keypads: &[&Box<dyn Keypad>], cache: &mut HashMap<(String, usize), usize>) -> usize {
    let key = (code.to_string(), keypads.len());
    
    if let Some(result) = cache.get(&key) {
        return *result;
    }
    
    if let Some(keypad) = keypads.first() {
        let mut position = keypad.start();
        let mut cost = 0;

        for key in code.chars() {
            let end = keypad.to_point(&key);
            let paths = build_paths(position, end);

            cost += paths.iter()
                .filter(|path| !path_includes(path, position, keypad.gap()))
                .map(|path| code_cost(path, &keypads[1..], cache))
                .min().unwrap();

            position = end;
        }

        cache.insert(key, cost);
        
        cost
    } else {
        code.len()
    }
}

fn main() {
    let door_keypad: Box<dyn Keypad> = Box::new(NumericKeypad {});
    let robot_keypad: Box<dyn Keypad> = Box::new(DirectionalKeypad {});
    let codes: Vec<_> = read_lines!("day-21/input.txt").collect();
    
    // Part 01
    let mut part01 = 0;
    let mut cache = HashMap::new();

    for code in &codes {
        let val = code[..3].parse::<i32>().unwrap();
        let cost = code_cost(code, &[&door_keypad, &robot_keypad, &robot_keypad], &mut cache) as i32;

        println!("{cost} * {val}");

        part01 += val * cost;
    }

    println!("part 01: {}", part01);
    
    // Part 02
    let mut part02 = 0;

    for code in &codes {
        let val = code[..3].parse::<usize>().unwrap();
        let cost = code_cost(
            code,
            &[&door_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad, &robot_keypad],
            &mut cache
        );

        println!("{cost} * {val}");

        part02 += val * cost;
    }

    println!("part 02: {}", part02);
}
