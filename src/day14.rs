use crate::utils::read_lines;
use std::collections::HashSet;

fn next_sand_point(sand_loc: (i32, i32), cave: &HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let down = (sand_loc.0, sand_loc.1 + 1);
    let left = (sand_loc.0 - 1, sand_loc.1 + 1);
    let right = (sand_loc.0 + 1, sand_loc.1 + 1);
    if !cave.contains(&down) {
        return Some(down);
    }
    if !cave.contains(&left) {
        return Some(left);
    }
    if !cave.contains(&right) {
        return Some(right);
    }
    None
}

fn part_1(bottom: i32, mut cave: HashSet<(i32, i32)>) -> i32 {
    let mut sand_units = 0;
    loop {
        let mut loc = (500, 0);
        while let Some(next_loc) = next_sand_point(loc, &cave) {
            loc = next_loc;
            if loc.1 > bottom {
                return sand_units;
            }
        }
        sand_units += 1;
        cave.insert(loc);
        if loc == (500, 0) {
            return sand_units;
        }
    }
}

fn part_2(mut cave: HashSet<(i32, i32)>) -> i32 {
    let mut sand_units = 0;
    loop {
        let mut loc = (500, 0);
        while let Some(next_loc) = next_sand_point(loc, &cave) {
            loc = next_loc;
        }
        sand_units += 1;
        cave.insert(loc);
        if loc == (500, 0) {
            return sand_units;
        }
    }
}

pub fn run() {
    let lines = read_lines("in/day14.in").unwrap();

    let paths: Vec<Vec<(i32, i32)>> = lines
        .map(|path| {
            path.unwrap()
                .split(" -> ")
                .map(|point| {
                    let mut iter = point.split(',');
                    (
                        iter.next().unwrap().parse::<i32>().unwrap(),
                        iter.next().unwrap().parse::<i32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut cave: HashSet<(i32, i32)> = HashSet::new();

    let mut bottom = 0;
    for path in paths {
        for window in path.windows(2) {
            bottom = std::cmp::max(bottom, window[0].1);
            bottom = std::cmp::max(bottom, window[1].1);
            if let [a, b] = window {
                if a.0 == b.0 {
                    for y in std::cmp::min(a.1, b.1)..=std::cmp::max(a.1, b.1) {
                        cave.insert((a.0, y));
                    }
                } else {
                    for x in std::cmp::min(a.0, b.0)..=std::cmp::max(a.0, b.0) {
                        cave.insert((x, a.1));
                    }
                }
            }
        }
    }

    println!("Day 14, part 1 {}", part_1(bottom, cave.clone()));

    let floor = bottom + 2;
    for x in -1000..=1000 {
        cave.insert((x, floor));
    }
    println!("Day 14, part 2 {}", part_2(cave.clone()));
}
