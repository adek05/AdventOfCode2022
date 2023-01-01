use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils::read_lines;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Empty,
    Start,
    Border,
    End,
    Blizzard(Direction),
}

impl Field {
    pub fn next(&self, basin: &Basin, loc: (usize, usize), time: usize) -> (usize, usize) {
        let x = basin[0].len();
        let y = basin.len();

        match self {
            Field::Empty => loc,
            Field::Start => loc,
            Field::End => loc,
            Field::Border => loc,
            Self::Blizzard(Direction::Left) => (1 + (loc.0 - 1 + (x - 3) * time) % (x - 2), loc.1),
            Self::Blizzard(Direction::Right) => (1 + (loc.0 - 1 + time) % (x - 2), loc.1),
            Self::Blizzard(Direction::Up) => (loc.0, 1 + (loc.1 - 1 + (y - 3) * time) % (y - 2)),
            Self::Blizzard(Direction::Down) => (loc.0, 1 + (loc.1 - 1 + time) % (y - 2)),
        }
    }
}

type Basin = Vec<Vec<Field>>;
type Blizzards = HashMap<(usize, usize), Vec<Field>>;

const STEPS: [(i32, i32); 5] = [(0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)];

fn blizzard_at_step(start_blizzards: &Blizzards, basin: &Basin, step: u32) -> Blizzards {
    let mut next_blizzards: Blizzards = HashMap::new();
    start_blizzards.clone().iter().for_each(|(loc, fields)| {
        for f in fields {
            let next = f.next(basin, *loc, step as usize);
            next_blizzards
                .entry(next)
                .and_modify(|x| x.push(*f))
                .or_insert_with(|| vec![*f]);
        }
    });
    next_blizzards
}

fn simulate(
    basin: Basin,
    blizzards: Blizzards,
    location: (usize, usize),
    time_start: u32,
    target: Field,
) -> u32 {
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();
    queue.push_back((location, time_start));

    let mut blizzards_at_t: HashMap<u32, Blizzards> = HashMap::new();
    for i in 0..300 {
        blizzards_at_t.insert(i as u32, blizzard_at_step(&blizzards, &basin, i as u32));
    }

    let mut x = 0;
    let mut visited: HashSet<(u32, (usize, usize))> = HashSet::new();
    loop {
        let (location, step_no) = queue.pop_front().unwrap();
        if visited.contains(&(step_no, location)) {
            continue;
        }
        visited.insert((step_no, location));
        x += 1;
        if x % 10000 == 0 {}
        if basin[location.1][location.0] == target {
            return step_no;
        }

        let next_blizzards = blizzards_at_t
            .get(&((step_no + 1) % blizzards_at_t.len() as u32))
            .unwrap();

        for (dx, dy) in STEPS {
            let (x, y) = (location.0 as i32 + dx, location.1 as i32 + dy);
            if y < 0 {
                continue;
            }
            if y >= basin.len() as i32 {
                continue;
            }
            if basin[y as usize][x as usize] == Field::Border {
                continue;
            }

            if !next_blizzards.contains_key(&(x as usize, y as usize)) {
                queue.push_back(((x as usize, y as usize), step_no + 1));
            }
        }
    }
}

pub fn run() {
    let lines = read_lines("in/day24.in").unwrap();

    let basin: Basin = lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Field::Border,
                    '.' => Field::Empty,
                    '>' => Field::Blizzard(Direction::Right),
                    '^' => Field::Blizzard(Direction::Up),
                    '<' => Field::Blizzard(Direction::Left),
                    'v' => Field::Blizzard(Direction::Down),
                    'S' => Field::Start,
                    'E' => Field::End,
                    _ => panic!("invalid character given"),
                })
                .collect()
        })
        .collect();

    let mut blizzards: HashMap<(usize, usize), Vec<Field>> = HashMap::new();
    basin.iter().enumerate().for_each(|(y, r)| {
        r.iter().enumerate().for_each(|(x, f)| {
            if matches!(f, Field::Blizzard(_)) {
                blizzards.insert((x, y), vec![*f]);
            }
        })
    });

    let start_to_end = simulate(basin.clone(), blizzards.clone(), (1, 0), 0, Field::End);
    let end_to_start = simulate(
        basin.clone(),
        blizzards.clone(),
        (basin[1].len() - 2, basin.len() - 1),
        start_to_end,
        Field::Start,
    );
    let start_to_end_2 = simulate(basin, blizzards, (1, 0), end_to_start, Field::End);
    println!("Day 24, part 1: {}", start_to_end);
    println!("Day 24, part 2: {}", start_to_end_2);
}
