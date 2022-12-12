use crate::utils::read_lines;
use std::collections::HashSet;
use std::collections::VecDeque;

fn is_valid(loc: &(i32, i32), grid: &[Vec<char>]) -> Option<(usize, usize)> {
    if loc.0 >= 0 && (loc.0 as usize) < grid.len() && loc.1 >= 0 && (loc.1 as usize) < grid[0].len()
    {
        return Some((loc.0 as usize, loc.1 as usize));
    }
    None
}

fn bfs(
    start: (usize, usize),
    dest_height: char,
    grid: &[Vec<char>],
    check: Box<dyn Fn(i32, i32) -> bool>,
) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((start, 0));
    while let Some((loc, distance)) = queue.pop_front() {
        if visited.contains(&loc) {
            continue;
        }
        visited.insert(loc);

        let current_height = grid[loc.0][loc.1];
        if current_height as char == dest_height {
            return distance;
        }

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let maybe_next_loc = (loc.0 as i32 + dx, loc.1 as i32 + dy);
            if let Some(next_loc) = is_valid(&maybe_next_loc, grid) {
                let next_height = grid[next_loc.0][next_loc.1] as i32;
                if check(next_height, current_height as i32) {
                    queue.push_back((next_loc, distance + 1));
                }
            }
        }
    }
    panic!("Could not find path");
}

pub fn run() {
    let lines = read_lines("in/day12.in");

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: Vec<Vec<char>> = vec![];
    let mut distance: Vec<Vec<u32>> = vec![];
    for line in lines.unwrap() {
        grid.push(line.unwrap().chars().collect());
        distance.push(vec![1_000_000_000; grid[0].len()]);
    }

    for (x, line) in grid.iter().enumerate() {
        if let Some((y, _)) = line.iter().enumerate().find(|x| *x.1 == 'S') {
            start = (x, y);
        }
        if let Some((y, _)) = line.iter().enumerate().find(|x| *x.1 == 'E') {
            end = (x, y);
        }
    }
    grid[start.0][start.1] = 'a';
    grid[end.0][end.1] = 'z';

    println!(
        "Day 12, part 1 {}",
        bfs(start, 'z', &grid, Box::new(|next, cur| next - cur <= 1))
    );
    println!(
        "Day 12, part 2 {}",
        bfs(end, 'a', &grid, Box::new(|next, cur| cur - next <= 1))
    );
}
