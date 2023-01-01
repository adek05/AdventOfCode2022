use crate::utils::read_lines;

pub fn to_digit(snafu: String) -> i64 {
    let mut power = 1;
    let mut res = 0;
    for c in snafu.chars().rev() {
        res += power * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("")
        };
        power *= 5;
    }
    res
}

/// ```
/// use aoc::day25::{to_snafu, to_digit};
/// assert_eq!(to_snafu(1), "1".to_string());
/// assert_eq!(to_snafu(2), "2".to_string());
/// assert_eq!(to_snafu(3), "1=".to_string());
/// assert_eq!(to_snafu(2022), "1=11-2".to_string());
/// ```
pub fn to_snafu(number: i64) -> String {
    let mut n = number;
    let mut res = "".to_owned();
    let lookup = ['0', '1', '2', '=', '-'];
    while n > 0 {
        let reminder = n% 5;
        res.push(lookup[reminder as usize]);
        n = n / 5 + i64::from(reminder > 2);
    }
    res.chars().rev().collect()
}

pub fn run() {
    let lines = read_lines("in/day25.in").unwrap();
    println!("Day 25: {}", to_snafu(lines.map(|l| to_digit(l.unwrap())).sum()));
}