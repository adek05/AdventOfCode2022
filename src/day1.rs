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
    let mut max = 0;
    let mut cur = 0;
    for line in lines {
        let l = line.unwrap();
        if l.is_empty() {
            max = std::cmp::max(max, cur);
            cur = 0;
        } else {
            cur += str::parse::<i64>(&l).unwrap();
        }
    }
    println!("Day 1.a: {}", max);
}