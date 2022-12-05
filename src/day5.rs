use crate::utils::read_lines;
use std::collections::HashMap;

struct Move {
    pub from: u32,
    pub to: u32,
    pub how_many: usize,
}

fn part1(mut crates: HashMap<u32, Vec<char>>, instructions: &[Move]) -> HashMap<u32, Vec<char>> {
    for instruction in instructions {
        let mut from = crates.get(&instruction.from).unwrap().clone();
        let mut to = crates.get(&instruction.to).unwrap().clone();
        for _ in 0..instruction.how_many {
            to.push(from.pop().unwrap());
        }
        crates.insert(instruction.from, from);
        crates.insert(instruction.to, to);
    }
    crates
}

fn part2(mut crates: HashMap<u32, Vec<char>>, instructions: &[Move]) -> HashMap<u32, Vec<char>> {
    for instruction in instructions {
        let mut from = crates.get(&instruction.from).unwrap().clone();
        let mut to = crates.get(&instruction.to).unwrap().clone();

        to.extend(&from[from.len() - instruction.how_many..from.len()]);
        from = from[0..from.len() - instruction.how_many].to_vec();
        crates.insert(instruction.from, from);
        crates.insert(instruction.to, to);
    }
    crates
}

pub fn what_is_on_top(crates: HashMap<u32, Vec<char>>) -> String {
    let mut res: String = "".to_string();
    for i in 1..10 {
        if let Some(top) = crates.get(&i).unwrap().last() {
            res.push(*top);
        } else {
            res.push(' ');
        }
    }
    res
}

pub fn run() {
    let lines = read_lines("in/day5.in").unwrap();
    let input: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let mut slices = input.split(|line| line.is_empty());
    let mut crates_in = slices.next().unwrap();
    let instructions_in = slices.next().unwrap();

    crates_in = &crates_in[0..crates_in.len() - 1];

    let mut crates: HashMap<u32, Vec<char>> = HashMap::new();

    for l in crates_in.iter().rev() {
        let mut idx = 1;
        let mut stack = 1;
        while idx < l.len() {
            let current_crate = l.chars().nth(idx).unwrap();
            if current_crate != ' ' {
                crates
                    .entry(stack)
                    .and_modify(|s| s.push(current_crate))
                    .or_insert_with(|| vec![current_crate]);
            }

            stack += 1;
            idx += 4;
        }
    }

    let mut instructions = vec![];
    for instruction in instructions_in {
        scan!(instruction; ("move", let how_many: usize, "from", let from:u32, "to", let to: u32) => {
            instructions.push(Move{from, to,  how_many});
        }
        ).unwrap();
    }

    println!(
        "Day 5, part 1: {}",
        what_is_on_top(part1(crates.clone(), &instructions))
    );
    println!(
        "Day 5, part 2: {}",
        what_is_on_top(part2(crates.clone(), &instructions))
    );
}
