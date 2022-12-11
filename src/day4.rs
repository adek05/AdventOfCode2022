use crate::utils::read_lines;

pub struct Elf {
    pub start: i32,
    pub end: i32,
}

/// ```
/// use aoc::day4::{contains, Elf};
/// assert_eq!(contains(&Elf{start:2, end: 4}, &Elf{start:6, end: 8}), false);
/// assert_eq!(contains(&Elf{start:6, end: 6}, &Elf{start:4, end: 6}), true);
/// ```
pub fn contains(r1: &Elf, r2: &Elf) -> bool {
    contains_impl(r1, r2) || contains_impl(r2, r1)
}

fn contains_impl(r1: &Elf, r2: &Elf) -> bool {
    if r1.start <= r2.start && r2.end <= r1.end {
        return true;
    }
    false
}

pub fn overlaps(r1: &Elf, r2: &Elf) -> bool {
    r1.start <= r2.start && r2.start <= r1.end
        || r2.start <= r1.start && r1.start <= r2.end
        || contains(r1, r2)
}

pub fn run() {
    let lines = read_lines("in/day4.in").unwrap();

    let mut fully_contains_pairs = 0;
    let mut overlap_pairs = 0;

    for line in lines {
        let l = line.unwrap();
        scan!(&l; (let s1: i32, "-", let e1:i32, ",", let s2: i32, "-", let e2: i32) => {
            let pair1 = Elf{start: s1, end: e1};
            let pair2 = Elf{start: s2, end: e2};
            if contains(&pair1, &pair2) {
                fully_contains_pairs += 1;
            }
            if overlaps(&pair1, &pair2) {
                overlap_pairs += 1;
            }
        })
        .unwrap();
    }

    println!("Day 4, part 1: {}", fully_contains_pairs);
    println!("Day 4, part 2: {}", overlap_pairs);
}
