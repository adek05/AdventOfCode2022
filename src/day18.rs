use crate::utils::read_lines;

use std::collections::HashSet;
use std::collections::VecDeque;

type Cube = (i32, i32, i32);

fn adjacent((x, y, z): Cube) -> [Cube; 6] {
    [
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ]
}

fn is_in(c: &Cube, xs: &(i32, i32), ys: &(i32, i32), zs: &(i32, i32)) -> bool {
    xs.0 <= c.0 && c.0 <= xs.1 && ys.0 <= c.1 && c.1 <= ys.1 && zs.0 <= c.2 && c.2 <= zs.1
}

pub fn run() {
    let lines = read_lines("in/day18.in").unwrap();
    let cubes: HashSet<Cube> = lines
        .map(|line| {
            let l = line.unwrap();
            scan!(&l; (let x: i32, ",", let y: i32, ",", let z: i32) => (x, y, z)).unwrap()
        })
        .collect();

    let mut surface_area = 0;

    for cube in &cubes {
        for adjacent in adjacent(*cube) {
            if !cubes.contains(&adjacent) {
                surface_area += 1;
            }
        }
    }
    println!("Day 18, part 1: {}", surface_area);

    let mut outer_surface_area = 0;
    let xs = (cubes.iter().map(|c| c.0).min().unwrap() - 2, cubes.iter().map(|c| c.0).max().unwrap() + 2);
    let ys = (cubes.iter().map(|c| c.1).min().unwrap() - 2, cubes.iter().map(|c| c.1).max().unwrap() + 2);
    let zs = (cubes.iter().map(|c| c.2).min().unwrap() - 2, cubes.iter().map(|c| c.2).max().unwrap() + 2);
    let mut visited: HashSet<Cube> = HashSet::new();
    let mut q: VecDeque<Cube> = VecDeque::new();
    q.push_back((xs.0, ys.0, zs.0));
    while let Some(next) = q.pop_front() {
        if cubes.contains(&next) {
            outer_surface_area += 1;
        }
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next.clone());
        if !is_in(&next, &xs, &ys, &zs) {
            continue;
        }
        if !cubes.contains(&next) {
            q.extend(adjacent(next));
        }
    }
    println!("Day 18, part 2: {}", outer_surface_area);

}
