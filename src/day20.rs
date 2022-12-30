use crate::utils::read_lines;

fn idx(number: i64, offset: usize, list: &Vec<(usize, i64)>) -> i64 {
    let p = list.iter().position(|x| x.1 == number).unwrap();
    list[(p + offset) % list.len()].1
}

fn mix(mut numbers: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    for i in 0..numbers.len() {
        let pos = numbers.iter().position(|x| x.0 == i).unwrap();
        let number = numbers[pos];

        let modulo = (numbers.len() - 1) as i64;
        let new_pos = ((1 + pos as i64 - 1 + (number.1 % modulo) + modulo) % modulo) as usize;
        let mut new: Vec<(usize, i64)> = vec![];
        new.extend(&numbers[0..pos]);
        new.extend(&numbers[pos + 1..]);

        let mut new_numbers: Vec<(usize, i64)> = vec![];
        new_numbers.extend(&new[0..new_pos]);
        new_numbers.push(number);
        new_numbers.extend(&new[new_pos..]);
        numbers = new_numbers;
    }
    numbers
}

pub fn run() {
    let lines = read_lines("in/day20.in").unwrap();
    let numbers: Vec<(usize, i64)> = lines
        .enumerate()
        .map(|(idx, line)| (idx, line.unwrap().parse::<i64>().unwrap()))
        .collect();
    {
        let mixed = mix(numbers.clone());
        println!(
            "Part 1: {}",
            idx(0, 1000, &mixed) + idx(0, 2000, &mixed) + idx(0, 3000, &mixed)
        )
    }
    {
        let mut mixed: Vec<(usize, i64)> = numbers
            .iter()
            .cloned()
            .map(|x| (x.0, x.1 * 811589153))
            .collect();
        for _ in 0..10 {
            mixed = mix(mixed);
        }
        println!(
            "Part 2: {}",
            idx(0, 1000, &mixed) + idx(0, 2000, &mixed) + idx(0, 3000, &mixed)
        );
    }
}
