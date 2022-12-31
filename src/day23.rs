use crate::utils::read_lines;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i32, i32);

fn apply_move(p: &Point, d: &(i32, i32)) -> Point {
    (p.0 + d.0, p.1 + d.1)
}

pub fn run() {
    let lines = read_lines("in/day23.in").unwrap();

    let all_moves = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut candidate_moves = VecDeque::from_iter(&[
        ([(0, -1), (-1, -1), (1, -1)], (0, -1)),
        ([(0, 1), (-1, 1), (1, 1)], (0, 1)),
        ([(-1, 0), (-1, -1), (-1, 1)], (-1, 0)),
        ([(1, 0), (1, -1), (1, 1)], (1, 0)),
    ]);

    let mut elves: HashSet<Point> = lines
        .enumerate()
        .flat_map(|(y, row)| {
            row.unwrap()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect();

    let mut round_id = 1;
    loop {
        let mut proposed_locations: HashMap<Point, usize> = HashMap::new();

        let mut next_for_elf: HashMap<Point, Point> = HashMap::new();

        for elf in &elves {
            if all_moves
                .iter()
                .all(|d| !elves.contains(&apply_move(elf, d)))
            {
                proposed_locations.insert(*elf, 1);
                next_for_elf.insert(*elf, *elf);
                continue;
            }

            for (moves, dd) in &candidate_moves {
                if moves.iter().all(|d| !elves.contains(&apply_move(elf, d))) {
                    let proposed = apply_move(elf, dd);
                    proposed_locations
                        .entry(proposed)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                    next_for_elf.insert(*elf, proposed);
                    break;
                };
            }

            if !next_for_elf.contains_key(elf) {
                proposed_locations
                    .entry(*elf)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                next_for_elf.insert(*elf, *elf);
            }
        }
        assert_eq!(next_for_elf.len(), elves.len());

        let new_elves = HashSet::from_iter(next_for_elf.iter().map(|(cur, next)| {
            if proposed_locations.get(next).unwrap() > &1 {
                *cur
            } else {
                *next
            }
        }));

        if elves == new_elves {
            println!("Day 23, part 2: {}", round_id);
            break;
        }
        elves = new_elves;
        if round_id == 10 {
            let min_x = elves.iter().map(|elf| elf.0).min().unwrap();
            let max_x = elves.iter().map(|elf| elf.0).max().unwrap();
            let min_y = elves.iter().map(|elf| elf.1).min().unwrap();
            let max_y = elves.iter().map(|elf| elf.1).max().unwrap();

            println!(
                "Day 23, part 1: {}",
                (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
            );
        }

        // Rotate moves
        let x = candidate_moves.pop_front().unwrap();
        candidate_moves.push_back(x);

        round_id += 1;
    }
}
