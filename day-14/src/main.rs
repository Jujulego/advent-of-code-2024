use std::collections::HashMap;
use nalgebra::{point, vector, Point2, Vector2};
use owo_colors::{AnsiColors, OwoColorize, Style};

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const AREA_HEIGHT: i32 = 103;
const AREA_WIDTH: i32 = 101;

fn parse_point(txt: &str) -> Point2<i32> {
    let mut numbers = txt.split(',')
        .map(|s| s.parse::<i32>().unwrap());

    point![numbers.next().unwrap(), numbers.next().unwrap()]
}

fn parse_vector(txt: &str) -> Vector2<i32> {
    let mut numbers = txt.split(',')
        .map(|s| s.parse::<i32>().unwrap());

    vector![numbers.next().unwrap(), numbers.next().unwrap()]
}

fn main() {
    let mut final_state = HashMap::new();

    for line in read_lines!("day-14/input.txt") {
        let mut parts = line.split_whitespace();
        let start = parse_point(&parts.next().unwrap()[2..]);
        let velocity = parse_vector(&parts.next().unwrap()[2..]);

        let end = start + 100 * velocity;
        let end = point![end.x.rem_euclid(AREA_WIDTH), end.y.rem_euclid(AREA_HEIGHT)];

        //println!("p={:?},v={:?} => {:?}", start, velocity, end);

        *final_state.entry(end).or_insert(0) += 1;
    }

    for y in 0..AREA_HEIGHT {
        for x in 0..AREA_WIDTH {
            let pt = point![x, y];
            let style = if x == AREA_WIDTH / 2 || y == AREA_HEIGHT / 2 {
                Style::new().color(AnsiColors::BrightBlack)
            } else {
                Style::new()
            };

            if let Some(count) = final_state.get(&pt) {
                print!("{}", count.style(style));
            } else {
                print!("{}", ".".style(style));
            }
        }

        println!();
    }

    let top_left = (0..AREA_HEIGHT / 2)
        .flat_map(|y| (0..AREA_WIDTH / 2).map(move |x| point![x, y]))
        .filter_map(|pt| final_state.get(&pt))
        .sum::<i32>();

    let top_right = (0..AREA_HEIGHT / 2)
        .flat_map(|y| ((AREA_WIDTH / 2) + 1..AREA_WIDTH).map(move |x| point![x, y]))
        .filter_map(|pt| final_state.get(&pt))
        .sum::<i32>();

    let bottom_left = ((AREA_HEIGHT / 2) + 1..AREA_HEIGHT)
        .flat_map(|y| (0..AREA_WIDTH / 2).map(move |x| point![x, y]))
        .filter_map(|pt| final_state.get(&pt))
        .sum::<i32>();

    let bottom_right = ((AREA_HEIGHT / 2) + 1..AREA_HEIGHT)
        .flat_map(|y| ((AREA_WIDTH / 2) + 1..AREA_WIDTH).map(move |x| point![x, y]))
        .filter_map(|pt| final_state.get(&pt))
        .sum::<i32>();

    println!("{} {}", top_left, top_right);
    println!("{} {}", bottom_left, bottom_right);

    println!("part 01: {}", top_left * top_right * bottom_left * bottom_right);
}
