use crate::utils::read_lines;

const SIZE: usize = 50;

#[derive(Debug)]
enum Instruction {
    Clockwise,
    Counterclockwise,
    Forward(u32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Orientation {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    orientation_idx: usize,
}

const ORIENTATIONS: [Orientation; 4] = [
    Orientation::Right,
    Orientation::Down,
    Orientation::Left,
    Orientation::Up,
];

impl Position {
    pub fn forward(&self, map: &Vec<Vec<MapLocation>>) -> Position {
        let (dimx, dimy) = get_dimensions(map);

        let mut new_pos = (self.x, self.y);
        loop {
            new_pos = match ORIENTATIONS[self.orientation_idx] {
                Orientation::Right => ((new_pos.0 + 1) % dimx, new_pos.1),
                Orientation::Down => (new_pos.0, (new_pos.1 + 1) % dimy),
                Orientation::Left => ((new_pos.0 + dimx - 1) % dimx, new_pos.1),
                Orientation::Up => (new_pos.0, (dimy + new_pos.1 - 1) % dimy),
            };
            if map[new_pos.1][new_pos.0] != MapLocation::Padding {
                break;
            }
        }

        Position {
            x: new_pos.0,
            y: new_pos.1,
            orientation_idx: self.orientation_idx,
        }
    }

    pub fn forward2(&self, map: &[Vec<MapLocation>]) -> Position {
        let old_x = self.x;
        let old_y = self.y;
        let orientation = ORIENTATIONS[self.orientation_idx];
        let (x, y) = match ORIENTATIONS[self.orientation_idx] {
            Orientation::Right => (old_x + 1, old_y),
            Orientation::Down => (old_x, old_y + 1),
            Orientation::Left => (old_x - 1, old_y),
            Orientation::Up => (old_x, old_y - 1),
        };
        if map[y][x] != MapLocation::Padding {
            return Position {
                x,
                y,
                orientation_idx: self.orientation_idx,
            };
        }
        let res = if y == 0 {
            assert_eq!(orientation, Orientation::Up);
            if x <= 100 {
                // 1U -> 6R
                Position {
                    x: 1,
                    y: 151 + (x - 1) % 50,
                    orientation_idx: 0, // Right
                }
            } else {
                // 2U -> 6U
                Position {
                    x: 1 + (x - 1) % 50,
                    y: 200,
                    orientation_idx: 3, // Up
                }
            }
        } else if x == 151 {
            assert_eq!(orientation, Orientation::Right);
            // 2R -> 5L
            Position {
                x: 100,
                y: 150 - (y - 1) % 50,
                orientation_idx: 2, // Left
            }
        } else if x > 100 && y > 50 {
            // 2D -> 3L
            if orientation == Orientation::Down {
                Position {
                    x: 100,
                    y: 51 + (x - 1) % 50,
                    orientation_idx: 2, // Left
                }
            } else if y <= 100 {
                // 3R -> 2U
                assert_eq!(orientation, Orientation::Right);
                Position {
                    x: 101 + (y - 1) % 50,
                    y: 50,
                    orientation_idx: 3, // Up
                }
            } else {
                assert!((101..=150).contains(&y));
                assert_eq!(orientation, Orientation::Right);
                // 5R -> 2L
                Position {
                    x: 150,
                    y: 50 - (y - 1) % 50,
                    orientation_idx: 2, // Left
                }
            }
        } else if y > 150 && x > 50 {
            if orientation == Orientation::Down {
                // 5D -> 6L
                Position {
                    x: 50,
                    y: 151 + (x - 1) % 50,
                    orientation_idx: 2, // Left
                }
            } else {
                // 6R -> 5U
                assert!(x > 50);
                assert_eq!(orientation, Orientation::Right);
                Position {
                    x: 51 + (y - 1) % 50,
                    y: 150,
                    orientation_idx: 3, // Up
                }
            }
        } else if y > 200 {
            // 6D -> 2D
            Position {
                x: 101 + (x - 1) % 50,
                y: 1,
                orientation_idx: 1, // Down
            }
        } else if x == 0 {
            if y >= 151 {
                // 6L -> 1D
                Position {
                    x: 51 + (y - 1) % 50,
                    y: 1,
                    orientation_idx: 1, // Down
                }
            } else {
                // 4L -> 1R
                assert!(y >= 101);
                Position {
                    x: 51,
                    y: 50 - (y - 1) % 50,
                    orientation_idx: 0, // Right
                }
            }
        } else {
            assert!(y <= 100 && x <= 50);
            if orientation == Orientation::Up {
                // 4U -> 3R
                Position {
                    x: 51,
                    y: 51 + (x - 1) % 50,
                    orientation_idx: 0, //Right
                }
            } else {
                assert_eq!(orientation, Orientation::Left);
                if y <= 50 {
                    // 1L -> 4R
                    Position {
                        x: 1,
                        y: 150 - (y - 1) % 50,
                        orientation_idx: 0, // Right
                    }
                } else {
                    // 3L -> 4D
                    assert!(y <= 100 && x <= 50);
                    Position {
                        x: 1 + (y - 1) % 50,
                        y: 101,
                        orientation_idx: 1, // Down
                    }
                }
            }
        };
        if Orientation::Left == ORIENTATIONS[res.orientation_idx] {
            assert_eq!(res.x % 50, 0);
        }
        if Orientation::Right == ORIENTATIONS[res.orientation_idx] {
            assert_eq!(res.x % 50, 1);
        }
        if Orientation::Up == ORIENTATIONS[res.orientation_idx] {
            assert_eq!(res.y % 50, 0);
        }
        if Orientation::Down == ORIENTATIONS[res.orientation_idx] {
            assert_eq!(res.y % 50, 1);
        }
        res
    }

    pub fn clockwise(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            orientation_idx: (self.orientation_idx + 1) % 4,
        }
    }

    pub fn counter_clockwise(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            orientation_idx: (self.orientation_idx + 3) % 4,
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut res = vec![];

    let mut acc = String::new();
    for c in s.chars() {
        if c == 'L' || c == 'R' {
            res.push(Instruction::Forward(acc.parse::<u32>().unwrap()));
            acc.clear();
            if c == 'L' {
                res.push(Instruction::Counterclockwise);
            } else {
                res.push(Instruction::Clockwise);
            }
        } else {
            acc.push(c);
        }
    }
    if acc != String::new() {
        res.push(Instruction::Forward(acc.parse::<u32>().unwrap()));
    }

    res
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MapLocation {
    Empty,
    Wall,
    Padding,
}

impl From<char> for MapLocation {
    fn from(c: char) -> MapLocation {
        match c {
            ' ' => MapLocation::Padding,
            '#' => MapLocation::Wall,
            '.' => MapLocation::Empty,
            x => panic!("Invalid map location: {}", x),
        }
    }
}

fn get_dimensions(grid: &Vec<Vec<MapLocation>>) -> (usize, usize) {
    (grid[0].len(), grid.len())
}

pub fn run() {
    let lines = read_lines("in/day22.in").unwrap();

    let mut input: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let instructions = parse_instructions(input.last().unwrap());
    input.truncate(input.len() - 2);

    let max_len = input.iter().map(|x| x.chars().count()).max().unwrap();

    let mut map: Vec<Vec<MapLocation>> = input
        .iter()
        .map(|row| row.chars().map(|c| c.into()).collect())
        .collect();

    // Pad map
    map.iter_mut()
        .for_each(|r| r.resize(max_len, MapLocation::Padding));

    let start_x = map[0]
        .iter()
        .position(|x| x == &MapLocation::Empty)
        .unwrap();

    {
        let mut current_pos = Position {
            x: start_x,
            y: 0,
            orientation_idx: 0,
        };
        for instruction in &instructions {
            assert!(map[current_pos.y][current_pos.x] == MapLocation::Empty);
            current_pos = match instruction {
                Instruction::Clockwise => current_pos.clockwise(),
                Instruction::Counterclockwise => current_pos.counter_clockwise(),
                Instruction::Forward(n) => {
                    for _ in 0..*n {
                        let next_pos = current_pos.forward(&map);
                        assert!(map[current_pos.y][current_pos.x] == MapLocation::Empty);
                        if map[next_pos.y][next_pos.x] == MapLocation::Wall {
                            break;
                        }
                        current_pos = next_pos;
                    }
                    assert!(map[current_pos.y][current_pos.x] == MapLocation::Empty);
                    current_pos
                }
            }
        }

        println!(
            "Day 22, part 1: {}",
            1000 * (current_pos.y + 1) + 4 * (current_pos.x + 1) + current_pos.orientation_idx
        );
    }

    // Part 2

    // Pad everything
    map.iter_mut().for_each(|r| {
        r.insert(0, MapLocation::Padding);
        r.push(MapLocation::Padding);
    });
    map.insert(0, [MapLocation::Padding; 3 * SIZE + 2].to_vec());
    map.push([MapLocation::Padding; 3 * SIZE + 2].to_vec());

    let mut current_pos = Position {
        x: start_x + 1,
        y: 1,
        orientation_idx: 0,
    };
    for instruction in &instructions {
        assert_eq!(map[current_pos.y][current_pos.x], MapLocation::Empty);
        current_pos = match instruction {
            Instruction::Clockwise => current_pos.clockwise(),
            Instruction::Counterclockwise => current_pos.counter_clockwise(),
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    let next_pos = current_pos.forward2(&map);
                    assert_eq!(map[current_pos.y][current_pos.x], MapLocation::Empty);
                    if map[next_pos.y][next_pos.x] == MapLocation::Wall {
                        break;
                    }
                    current_pos = next_pos;
                }
                assert_eq!(map[current_pos.y][current_pos.x], MapLocation::Empty);
                current_pos
            }
        }
    }

    println!(
        "Day 22, part 2: {}",
        1000 * current_pos.y + 4 * current_pos.x + current_pos.orientation_idx
    );
}
