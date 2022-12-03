use crate::utils::read_lines;
use std::collections::HashSet;

/// ```
/// use aoc::day3::priority;
/// assert_eq!(priority('p'), 16);
/// ```
pub fn priority(c: char) -> u32 {
    match c {
        c if c >='a' && c <='z' => 1 + c as u32 - 'a' as u32,
        c if c >='A' && c <='Z' => 27 + c as u32 - 'A' as u32,
        _ => panic!("Non-digit ASCII char given: {}", c),
    }
}

/// ```
/// use aoc::day3::find_bad_item;
/// assert_eq!(find_bad_item(&[1,2,3,4,4,5,6,4]), 4);
/// ```
pub fn find_bad_item(rucksack: &[u32]) -> u32 {
    let part1: HashSet<&u32> = HashSet::from_iter(&rucksack[0 .. rucksack.len() / 2]);
    let part2 = HashSet::from_iter(&rucksack[rucksack.len() / 2 ..]);
    **part1.intersection(&part2).next().unwrap()
}

pub fn run() {
    let lines = read_lines("in/day3.in").unwrap();

    let mut rucksacks: Vec<Vec<u32>> = vec![];
    
    for line in lines {
        let l = line.unwrap();
        rucksacks.push(l.chars().map(priority).collect());
    }
    println!("Day 3.1: {}", rucksacks.iter().map(|x| find_bad_item(&x)).sum::<u32>());

}