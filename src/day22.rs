use crate::utils::read_lines;

#[derive(Debug)]
enum Instruction {
    Clockwise,
    Counterclockwise,
    Forward(u32),
}

#[derive(Debug)]
enum Orientation {
    Right,
    Down,
    Left,
    Up,
}
// (1, 0), (0, 1), (-1, 0), ()

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
            if &map[new_pos.1][new_pos.0] != &MapLocation::Padding {
                break;
            }
        }

        Position {
            x: new_pos.0,
            y: new_pos.1,
            orientation_idx: self.orientation_idx,
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

// fn is_empty(map: &[String], p: &Position) -> bool {}

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
