use crate::utils::read_lines;
use std::collections::HashSet;

/// ```
/// use aoc::day6::part1;
/// assert_eq!(part1(Vec::from_iter("bvwbjplbgvbhsrlpgdmjqwftvncz".chars()), 4), 5);
/// assert_eq!(part1(Vec::from_iter("nppdvjthqldpwncqszvftbrmjlhg".chars()), 4), 6);
/// assert_eq!(part1(Vec::from_iter("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars()), 4), 11);
/// assert_eq!(part1(Vec::from_iter("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars()), 14), 19);
/// ```
pub fn part1(communication: Vec<char>, window_size: usize) -> usize {
    let mut first_match =
        communication
            .windows(window_size)
            .enumerate()
            .filter(|(_idx, window)| {
                let s: HashSet<&char> = HashSet::from_iter(*window);
                s.len() == window_size
            });
    first_match.next().unwrap().0 + window_size
}

pub fn run() {
    let mut lines = read_lines("in/day6.in").unwrap();
    let communication = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    println!("Day 6, part 1: {}", part1(communication.clone(), 4));
    println!("Day 6, part 2: {}", part1(communication, 14));
}
