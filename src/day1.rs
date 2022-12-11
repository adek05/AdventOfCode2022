use crate::utils::read_lines;

pub fn run() {
    let lines = read_lines("in/day1.in").unwrap();

    let mut elves: Vec<i64> = Vec::from_iter(lines)
        .split(|line| line.as_ref().unwrap().is_empty())
        .map(|x| {
            x.iter()
                .map(|l| str::parse::<i64>(l.as_ref().unwrap()).unwrap())
                .sum::<i64>()
        })
        .collect();

    elves.sort();
    elves.reverse();
    println!("Day 1.a: {}", elves[0]);
    println!("Day 1.b: {}", elves[0..3].iter().sum::<i64>());
}
