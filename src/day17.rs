use crate::utils::read_lines;

use std::collections::HashMap;

type Grid = Vec<[bool; 7]>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Flat,
    Star,
    Lshaped,
    Tall,
    Box,
}

#[derive(Debug)]
struct Rock {
    left: usize,
    bottom: usize,
    shape: Shape,
}

fn collision_tall(height: usize, pos: usize, bottom: usize, grid: &Grid) -> bool {
    for i in 0..height {
        if grid[bottom + i][pos] {
            return true;
        }
    }
    false
}

fn collision_flat(len: usize, left: usize, bottom: usize, grid: &Grid) -> bool {
    for i in 0..len {
        if grid[bottom][left + i] {
            return true;
        }
    }
    false
}

impl Rock {
    fn shift_right(&self) -> Self {
        match self.shape {
            Shape::Flat => Self {
                left: 3.min(self.left + 1),
                bottom: self.bottom,
                shape: self.shape,
            },
            Shape::Star => Self {
                left: 4.min(self.left + 1),
                bottom: self.bottom,
                shape: self.shape,
            },
            Shape::Lshaped => Self {
                left: 4.min(self.left + 1),
                bottom: self.bottom,
                shape: self.shape,
            },
            Shape::Tall => Self {
                left: 6.min(self.left + 1),
                bottom: self.bottom,
                shape: self.shape,
            },
            Shape::Box => Self {
                left: 5.min(self.left + 1),
                bottom: self.bottom,
                shape: self.shape,
            },
        }
    }

    fn shift_left(&self) -> Self {
        Self {
            left: if self.left > 0 {
                0.max(self.left - 1)
            } else {
                0
            },
            bottom: self.bottom,
            shape: self.shape,
        }
    }

    fn down(&self) -> Self {
        Self {
            left: self.left,
            bottom: self.bottom - 1,
            shape: self.shape,
        }
    }

    fn collision(&self, grid: &Grid) -> bool {
        match self.shape {
            Shape::Flat => collision_flat(4, self.left, self.bottom, grid),
            Shape::Star => {
                collision_flat(3, self.left, self.bottom + 1, grid)
                    || collision_tall(3, self.left + 1, self.bottom, grid)
            }
            Shape::Lshaped => {
                collision_flat(3, self.left, self.bottom, grid)
                    || collision_tall(3, self.left + 2, self.bottom, grid)
            }
            Shape::Tall => collision_tall(4, self.left, self.bottom, grid),
            Shape::Box => {
                collision_tall(2, self.left, self.bottom, grid)
                    || collision_tall(2, self.left + 1, self.bottom, grid)
            }
        }
    }

    fn add(&self, grid: &mut [[bool; 7]]) -> usize {
        match self.shape {
            Shape::Flat => {
                for i in self.left..self.left + 4 {
                    grid[self.bottom][i] = true;
                }
                1
            }
            Shape::Star => {
                grid[self.bottom][self.left + 1] = true;
                for i in self.left..self.left + 3 {
                    grid[self.bottom + 1][i] = true;
                }
                grid[self.bottom + 2][self.left + 1] = true;
                3
            }
            Shape::Lshaped => {
                for i in self.left..self.left + 3 {
                    grid[self.bottom][i] = true;
                }
                grid[self.bottom + 1][self.left + 2] = true;
                grid[self.bottom + 2][self.left + 2] = true;
                3
            }
            Shape::Tall => {
                for i in 0..4 {
                    grid[i + self.bottom][self.left] = true;
                }
                4
            }
            Shape::Box => {
                for i in 0..2 {
                    grid[self.bottom][self.left + i] = true;
                    grid[self.bottom + 1][self.left + i] = true;
                }
                2
            }
        }
    }
}

fn get_key(last_flat: usize, start: usize, grid: &Grid) -> String {
    let mut key: String = "".to_string();
    // for row in last_flat..start {
    for item in grid.iter().take(start).skip(last_flat) {
        key += &item
            .iter()
            .map(|x| if *x { "#" } else { "." })
            .collect::<String>();
    }
    key
}

pub fn run() {
    let mut lines = read_lines("in/day17.in").unwrap();
    let wind: Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    let mut grid: Vec<[bool; 7]> = vec![
        [true; 7], [false; 7], [false; 7], [false; 7], [false; 7], [false; 7], [false; 7],
        [false; 7], [false; 7], [false; 7], [false; 7],
    ];
    let mut tower_height = 1;
    let elems = vec![
        Shape::Flat,
        Shape::Star,
        Shape::Lshaped,
        Shape::Tall,
        Shape::Box,
    ];
    let mut wind_pos = 0;
    let mut prev_flat = 0;
    let mut cycle_height: u64 = 0;

    let mut flats: HashMap<(usize, String), (usize, usize)> = HashMap::new();

    let mut i = 0;
    let total_cycles = 1_000_000_000_000;
    while i < total_cycles {
        let mut rock = Rock {
            left: 2,
            bottom: tower_height + 3,
            shape: elems[i % elems.len()],
        };
        let wind_start = wind_pos;
        let tower_height_start = tower_height;
        let key = if prev_flat > 0 {
            Some(get_key(prev_flat, rock.bottom, &grid))
        } else {
            None
        };
        if rock.shape == Shape::Flat && key.is_some() {
            let k = key.clone().unwrap();

            if let Some((repeat_idx, last_height)) = flats.get(&(wind_start, k.clone())) {
                if cycle_height == 0 {
                    let cycle_len = i - repeat_idx;
                    let height_d = tower_height - last_height;
                    let n_cycles = (total_cycles - i) / cycle_len;
                    i += n_cycles * cycle_len;
                    cycle_height = height_d as u64 * n_cycles as u64;
                }
            }
        }
        loop {
            // Wind
            let shifted_rock = if wind[wind_pos] == '<' {
                rock.shift_left()
            } else {
                rock.shift_right()
            };
            if !shifted_rock.collision(&grid) {
                rock = shifted_rock;
            }
            wind_pos += 1;
            wind_pos %= wind.len();

            // Down
            let rock_down = rock.down();

            if rock_down.collision(&grid) {
                let height = rock.add(&mut grid);
                tower_height = tower_height.max(height + rock.bottom);
                for _ in grid.len()..tower_height + 7 {
                    grid.push([false; 7]);
                }
                break;
            }
            rock = rock_down;
        }
        if rock.shape == Shape::Flat {
            prev_flat = rock.bottom;
            if let Some(k) = key {
                flats.insert((wind_start, k), (i, tower_height_start));
            }
        }
        i += 1;
    }
    println!("Day 17, part 1: {}", tower_height as u64 - 1 + cycle_height);
}
