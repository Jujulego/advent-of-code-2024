use std::collections::{HashMap, HashSet};
use std::time::Instant;

macro_rules! read_lines {
    ($file:literal) => {{
        let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
        let buffer = std::io::BufReader::new(file);
        std::io::BufRead::lines(buffer).map(|line| line.unwrap())
    }};
}

fn main() {
    let mut links = HashMap::new();

    for line in read_lines!("day-23/input.txt") {
        let mut parts = line.split('-');
        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap().to_string();

        links.entry(a.clone()).or_insert_with(HashSet::new)
            .insert(b.clone());

        links.entry(b).or_insert_with(HashSet::new)
            .insert(a);
    }
    
    // Part 01
    let now = Instant::now();
    let pairs = links.iter()
        .flat_map(|(a, to)| to.iter().map(move |b| (a, b)));
   
    let mut groups = HashSet::new();
    
    for (a, b) in pairs {
        let to_a = links.get(a).unwrap();
        let to_b = links.get(b).unwrap();
        
        for c in to_a.intersection(to_b) {
            let mut group = [a, b, c];
            group.sort();
            
            groups.insert(group);
        }
    }
    
    let part01 = groups.iter()
        .filter(|grp| grp.iter().any(|c| c.starts_with("t")))
        .count();
    
    println!("part 01: {} ({:.2?})", part01, now.elapsed());
}
