use crate::utils::read_lines;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct HeadMove {
    pub direction: Direction,
    pub distance: u32,
}

pub fn move_head(pos: (i32, i32), head_move: &HeadMove) -> (i32, i32) {
    match head_move.direction {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
    }
}

/// ```
/// use aoc::day9::find_tail_position;
/// assert_eq!(find_tail_position((1, 2), (0, 0)), (1, 1));
/// assert_eq!(find_tail_position((2, 1), (0, 0)), (1, 1));
pub fn find_tail_position(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    if (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1 {
        return tail_pos;
    }
    [
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ]
    .iter()
    .map(|step| {
        let new_tail_pos = (tail_pos.0 + step.0, tail_pos.1 + step.1);
        let head_dist = (head_pos.0 - new_tail_pos.0) * (head_pos.0 - new_tail_pos.0)
            + (head_pos.1 - new_tail_pos.1) * (head_pos.1 - new_tail_pos.1);
        (head_dist, new_tail_pos)
    })
    .filter(|x| x.0 != 0)
    .min()
    .unwrap()
    .1
}

pub fn run() {
    let lines = read_lines("in/day9.in").unwrap();

    let mut steps = vec![];
    for line in lines {
        let l = line.unwrap();
        steps.push(
            scan!(&l;
                ("U", let distance: u32) => HeadMove{direction: Direction::Up, distance},
                ("D", let distance: u32) => HeadMove{direction: Direction::Down, distance},
                ("L", let distance: u32) => HeadMove{direction: Direction::Left, distance},
                ("R", let distance: u32) => HeadMove{direction: Direction::Right, distance},
            )
            .unwrap(),
        );
    }

    let mut head_position = (0, 0);
    let mut tail_position = vec![(0, 0); 1];
    let mut visited_positions = HashSet::new();
    visited_positions.insert((0, 0));

    for step in steps {
        for _ in 0..step.distance {
            head_position = move_head(head_position, &step);
            let mut tmp_head_position = head_position;
            for idx in 0..tail_position.len() {
                let tail_pos = tail_position[idx];
                tail_position[idx] = find_tail_position(tmp_head_position, tail_pos);
                assert!(
                    (tmp_head_position.0 - tail_position[idx].0).abs() <= 1
                        && (tmp_head_position.1 - tail_position[idx].1).abs() <= 1
                );
                tmp_head_position = tail_position[idx];
            }
            visited_positions.insert(tmp_head_position);
        }
    }

    println!("Day 9, part 1 and 2: {}", visited_positions.len());
}
