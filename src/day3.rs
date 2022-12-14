use crate::utils::read_lines;
use std::collections::HashSet;

/// ```
/// use aoc::day3::priority;
/// assert_eq!(priority('p'), 16);
/// ```
pub fn priority(c: char) -> u32 {
    match c {
        c if ('a'..='z').contains(&c) => 1 + c as u32 - 'a' as u32,
        c if ('A'..='Z').contains(&c) => 27 + c as u32 - 'A' as u32,
        _ => panic!("Non-digit ASCII char given: {}", c),
    }
}

/// ```
/// use aoc::day3::find_bad_item;
/// assert_eq!(find_bad_item(&[1,2,3,4,4,5,6,4]), 4);
/// ```
pub fn find_bad_item(rucksack: &[u32]) -> u32 {
    let part1: HashSet<&u32> = HashSet::from_iter(&rucksack[0..rucksack.len() / 2]);
    let part2 = HashSet::from_iter(&rucksack[rucksack.len() / 2..]);
    **part1.intersection(&part2).next().unwrap()
}

pub fn run() {
    let lines = read_lines("in/day3.in").unwrap();

    let mut rucksacks: Vec<Vec<u32>> = vec![];

    for line in lines {
        let l = line.unwrap();
        rucksacks.push(l.chars().map(priority).collect());
    }
    println!(
        "Day 3.1: {}",
        rucksacks.iter().map(|x| find_bad_item(x)).sum::<u32>()
    );

    let groups = rucksacks.chunks(3);

    let mut priorities_of_groups = vec![];
    for group in groups {
        priorities_of_groups.push(
            *group
                .iter()
                .cloned()
                .map(|rucksack| -> HashSet<u32> { HashSet::from_iter(rucksack) })
                .reduce(|acc, elem| acc.intersection(&elem).into_iter().cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap(),
        );
    }
    println!("Day 3.2: {}", priorities_of_groups.into_iter().sum::<u32>(),);
}
