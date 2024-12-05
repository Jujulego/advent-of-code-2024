use std::collections::{HashMap, HashSet};

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
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut rules_loaded = false;
    let mut part01 = 0;

    'lines: for line in read_lines!("day-05/input.txt") {
        if line.is_empty() {
            rules_loaded = true;
            continue;
        }

        if !rules_loaded {
            // Load rules
            let mut parts = line.split('|')
                .map(|p| p.parse::<i32>().unwrap());

            let x = parts.next().unwrap();
            let y = parts.next().unwrap();

            rules.entry(x)
                .or_default()
                .insert(y);
        } else {
            // Parse updates
            let mut pages = HashSet::new();
            let mut update = Vec::new();
            
            for page in line.split(',').map(|p| p.parse::<i32>().unwrap()) {
                if let Some(nexts) = rules.get(&page) {
                    if pages.intersection(nexts).count() > 0 { // <= previous pages includes next ones
                        continue 'lines; // <= invalid update go to next line
                    }
                }
                
                pages.insert(page);
                update.push(page);
            }
            
            part01 += update[update.len() / 2];
        }
    }

    println!("part 01: {part01}");
}
