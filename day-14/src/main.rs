use nalgebra::{point, vector, Point2, Vector2};
use std::collections::HashMap;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

const AREA_HEIGHT: i32 = 103;
const AREA_WIDTH: i32 = 101;

struct Bot {
    start: Point2<i32>,
    velocity: Vector2<i32>,
}

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

fn move_bots(bots: &[Bot], seconds: i32) -> HashMap<Point2<i32>, i32> {
    let mut result = HashMap::new();

    for bot in bots {
        let end = bot.start + seconds * bot.velocity;
        let end = point![end.x.rem_euclid(AREA_WIDTH), end.y.rem_euclid(AREA_HEIGHT)];

        *result.entry(end).or_insert(0) += 1;
    }

    result
}

fn print_map(bots: &HashMap<Point2<i32>, i32>) {
    for y in (0..AREA_HEIGHT).step_by(2) {
        for x in (0..AREA_WIDTH).step_by(2) {
            let tl = bots.contains_key(&point![x, y]);
            let tr = bots.contains_key(&point![x+1, y]);
            let br = bots.contains_key(&point![x+1, y+1]);
            let bl = bots.contains_key(&point![x, y+1]);

            match (tl, tr, br, bl) {
                (true, false, false, false) => print!("\u{2598}"),
                (true, true, false, false) => print!("\u{2580}"),
                (true, false, true, false) => print!("\u{259A}"),
                (true, false, false, true) => print!("\u{258C}"),
                (true, true, true, false) => print!("\u{259C}"),
                (true, true, false, true) => print!("\u{259B}"),
                (true, false, true, true) => print!("\u{2599}"),
                (true, true, true, true) => print!("\u{2588}"),
                (false, true, false, false) => print!("\u{259D}"),
                (false, true, true, false) => print!("\u{2590}"),
                (false, true, false, true) => print!("\u{259E}"),
                (false, true, true, true) => print!("\u{259F}"),
                (false, false, true, false) => print!("\u{2597}"),
                (false, false, true, true) => print!("\u{2584}"),
                (false, false, false, true) => print!("\u{2596}"),
                (false, false, false, false) => print!(" "),
            }
        }

        println!();
    }
}

fn main() {
    let bots = read_lines!("day-14/input.txt")
        .map(|line| {
            let mut parts = line.split_whitespace();

            Bot {
                start: parse_point(&parts.next().unwrap()[2..]),
                velocity: parse_vector(&parts.next().unwrap()[2..])
            }
        })
        .collect::<Vec<_>>();

    // Part 01
    let final_state = move_bots(&bots, 100);

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

    println!("part 01: {}", top_left * top_right * bottom_left * bottom_right);

    // Part 02
    let mut i = 0;

    loop {
        i += 1;

        let state = move_bots(&bots, i);
        let mut stop = false;

        for y in 0..AREA_HEIGHT {
            let mut last_empty = -1;

            for x in 0..AREA_WIDTH {
                let pt = point![x, y];

                if state.contains_key(&point![x, y]) {
                    if x - last_empty >= 10 {
                        stop = true;
                        break;
                    }
                } else {
                    last_empty = x;
                }
            }

            if stop {
                break;
            }
        }

        if i % 50 == 0 || stop {
            print!("\x1b[2J");
            print_map(&state);
            println!("i = {i}");
        }

        if stop {
            break;
        }
    }
}
