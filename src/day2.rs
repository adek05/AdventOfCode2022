use crate::utils::read_lines;

#[derive(PartialEq, Eq)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

impl Symbol {
    pub fn cmp(&self, other: &Symbol) -> Result {
        match (self, other) {
            (a, b) if a == b => Result::Draw,
            (Symbol::Rock, Symbol::Paper) => Result::Loss,
            (Symbol::Paper, Symbol::Scissors) => Result::Loss,
            (Symbol::Scissors, Symbol::Rock) => Result::Loss,
            _ => Result::Win,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }

    pub fn parse(c: char) -> Self {
        match c {
            'A' => Symbol::Rock,
            'B' => Symbol::Paper,
            'C' => Symbol::Scissors,
            'X' => Symbol::Rock,
            'Y' => Symbol::Paper,
            'Z' => Symbol::Scissors,
            _ => panic!("Invalid input give: {}", c),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Result {
    Draw,
    Loss,
    Win,
}

impl Result {
    pub fn value(&self) -> i32 {
        match self {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }

    pub fn parse(c: char) -> Self {
        match c {
            'X' => Result::Loss,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("Invalid result code given: {}", c),
        }
    }
}

fn score_round(opponent: &Symbol, me: &Symbol) -> i32 {
    me.cmp(opponent).value() + me.value()
}

pub fn run() {
    let options = vec![Symbol::Paper, Symbol::Scissors, Symbol::Rock];

    let lines = read_lines("in/day2.in").unwrap();
    let mut score_part_1 = 0;
    let mut score_part_2 = 0;
    for line in lines {
        let l = line.unwrap();

        let opponent = Symbol::parse(l.chars().next().unwrap());
        let me = Symbol::parse(l.chars().nth(2).unwrap());
        let expected_result = Result::parse(l.chars().nth(2).unwrap());

        score_part_1 += score_round(&opponent, &me);

        for me in &options {
            if me.cmp(&opponent) == expected_result {
                score_part_2 += score_round(&opponent, me);
            }
        }
    }

    println!("Day 2.a: {}", score_part_1);
    println!("Day 2.b: {}", score_part_2);
}
