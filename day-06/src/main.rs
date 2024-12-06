use std::collections::{HashMap, HashSet};
use std::iter::FusedIterator;
use nalgebra::{point, vector, Point2, Vector2};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

const UP: Vector2<i32> = vector![0, -1];
const RIGHT: Vector2<i32> = vector![1, 0];
const DOWN: Vector2<i32> = vector![0, 1];
const LEFT: Vector2<i32> = vector![-1, 0];

fn turn_right(dir: Vector2<i32>) -> Vector2<i32> {
    if dir == UP {
        RIGHT
    } else if dir == RIGHT {
        DOWN
    } else if dir == DOWN {
        LEFT
    } else if dir == LEFT {
        UP
    } else {
        unreachable!()
    }
}

/////////////////////////////////////////////////////////////////////
// GuardMap
/////////////////////////////////////////////////////////////////////
trait GuardMap {
    fn look_at(&self, point: &Point2<i32>) -> Option<&char>;
}

/////////////////////////////////////////////////////////////////////
// InputMap
/////////////////////////////////////////////////////////////////////
type InputMap = Vec<Vec<char>>;

impl GuardMap for InputMap {
    fn look_at(&self, point: &Point2<i32>) -> Option<&char> {
        self.get(point.y as usize).and_then(|row| row.get(point.x as usize))
    }
}

/////////////////////////////////////////////////////////////////////
// CorrectedMap
/////////////////////////////////////////////////////////////////////
struct CorrectedMap<'m> {
    map: &'m InputMap,
    object: Point2<i32>,
}

impl<'m> GuardMap for CorrectedMap<'m> {
    fn look_at(&self, point: &Point2<i32>) -> Option<&char> {
        if point == &self.object {
            return Some(&'#');
        }

        self.map.look_at(point)
    }
}

/////////////////////////////////////////////////////////////////////
// Guard
/////////////////////////////////////////////////////////////////////
struct Guard<'a, M: GuardMap> {
    map: &'a M,
    position: Point2<i32>,
    direction: Vector2<i32>,
}

impl<'a, M: GuardMap> Iterator for Guard<'a, M> {
    type Item = (Point2<i32>, Vector2<i32>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.position + self.direction;

            match self.map.look_at(&next) {
                Some(&'#') => self.direction = turn_right(self.direction),
                Some(_) => {
                    self.position = next;
                    return Some((next, self.direction));
                },
                None => return None,
            }
        }
    }
}

impl<'a, M: GuardMap> FusedIterator for Guard<'a, M> {}

/////////////////////////////////////////////////////////////////////
// main
/////////////////////////////////////////////////////////////////////
fn patrol<M: GuardMap>(map: &M, start: Point2<i32>) -> Option<HashMap<Point2<i32>, HashSet<Vector2<i32>>>> {
    let mut visited = HashMap::new();
    visited.insert(start, HashSet::from([UP]));

    let guard = Guard { map, position: start, direction: UP };

    for (pos, dir) in guard {
        let dirs = visited.entry(pos).or_default();

        if dirs.contains(&dir) {
            return None;
        } else {
            dirs.insert(dir);
        }
    }

    Some(visited)
}

fn main() {
    // Load map
    let map: Vec<Vec<char>> = read_lines!("day-06/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    // Search start
    let mut start = point![0, 0];

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                start = point![x as i32, y as i32];
            }
        }
    }

    // Part 1
    let visited = patrol(&map, start).unwrap();
    println!("part 01: {}", visited.len());

    // Part 2
    let part2 = visited.keys()
        .filter(|&pos| pos != &start)
        .filter(|&pos| {
            let map = CorrectedMap { map: &map, object: *pos };
            patrol(&map, start).is_none()
        })
        .count();

    println!("part 02: {part2}");
}
