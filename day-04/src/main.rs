extern crate nalgebra as na;

use na::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn get_letter<'a>(grid: &'a [Vec<char>], coord: &Point2<i32>) -> Option<&'a char> {
    grid.get(coord.y as usize)
        .and_then(|row| row.get(coord.x as usize))
}

fn extract_word(grid: &[Vec<char>], coord: &Point2<i32>, direction: &Vector2<i32>) -> String {
    (0..3)
        .map(move |i| coord + (i + 1) * direction)
        .filter_map(|coord| get_letter(grid, &coord))
        .copied()
        .collect()
}

fn main() {
    let directions = [
      vector![1, 0],
      vector![1, 1],
      vector![0, 1],
      vector![-1, 1],
      vector![-1, 0],
      vector![-1, -1],
      vector![0, -1],
      vector![1, -1]
    ];
    let x_moves = [
      vector![1, 1],
      vector![-1, 1],
      vector![-1, -1],
      vector![1, -1]
    ];

    let grid = read_lines!("day-04/input.txt")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part01 = 0;
    let mut part02 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, &letter)| letter == 'X') {
            let pt = point![x as i32, y as i32];

            part01 += directions.iter()
                .map(|dir| extract_word(&grid, &pt, dir))
                .filter(|word| *word == "MAS")
                .count();
        }

        for (x, _) in row.iter().enumerate().filter(|(_, &letter)| letter == 'A') {
            let pt = point![x as i32, y as i32];

            let x_word = x_moves.iter()
                .filter_map(|dir| get_letter(&grid, &(pt + dir)))
                .copied()
                .collect::<String>();

            match x_word.as_str() {
                "MMSS" | "SMMS" | "MSSM" | "SSMM" => {
                    part02 += 1;
                }
                _ => {}
            }
        }
    }

    println!("part 01: {}", part01);
    println!("part 02: {}", part02);
}
