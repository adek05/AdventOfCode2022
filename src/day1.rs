use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    let lines = read_lines("in/day1.in").unwrap();
    let mut elves = vec![];
    let mut cur = 0;
    for line in lines {
        let l = line.unwrap();
        if l.is_empty() {
            elves.push(cur);
            cur = 0;
        } else {
            cur += str::parse::<i64>(&l).unwrap();
        }
    }
    elves.sort();
    elves.reverse();
    println!("Day 1.a: {}", elves[0]);
    println!("Day 1.b: {}", elves[0..3].iter().sum::<i64>());
}