use crate::utils::read_lines;

#[derive(Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    pub fn eval(&self, arg: u64) -> u64 {
        match self {
            Op::Add(x) => arg + x,
            Op::Mul(x) => arg * x,
            Op::Square => arg * arg,
        }
    }
}

pub fn reduce_worry_level(level: u64) -> u64 {
    level / 1
}

pub fn reduce_worry_level_2(modulo: u64, level: u64) -> u64 {
    level % modulo
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub op: Op,
    pub test_divisble_by: u64,
    pub throw_to_if_true: usize,
    pub throw_to_if_false: usize,
    pub inspection_count: usize,
}

pub fn run() {
    let lines = read_lines("in/day11.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let mut modulo = 1;
    let mut monkeys: Vec<Monkey> = 
    lines.split(|l| l.is_empty()).into_iter().map(|monkey_lines| {
        let mut monkey_line = monkey_lines.iter();
        monkey_line.next();
        let monkey_items =
        scan!(monkey_line.next().unwrap(); ("  Starting items: ", [let items: u64],+: Vec<u64>) => items ).unwrap();
        let op =
        scan!(monkey_line.next().unwrap(); 
            ("  Operation: new = old + ", let delta: u64) => Op::Add(delta),
            ("  Operation: new = old * old") => Op::Square,
            ("  Operation: new = old * ", let delta: u64) => Op::Mul(delta) 
        ).unwrap();
        let divisible_by = scan!(monkey_line.next().unwrap();
            ("  Test: divisible by ", let divider: u64) => divider
        ).unwrap();
        let true_throw = scan!(monkey_line.next().unwrap();
            ("    If true: throw to monkey ", let monkey: usize) => monkey
        ).unwrap();
        let false_throw = scan!(monkey_line.next().unwrap();
            ("    If false: throw to monkey ", let monkey: usize) => monkey
        ).unwrap();
        modulo *= divisible_by;

        Monkey { items: monkey_items, op: op, test_divisble_by: divisible_by, throw_to_if_true: true_throw, throw_to_if_false: false_throw , inspection_count: 0}
    }).collect();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i].inspection_count += monkeys[i].items.len();

            let mut what_to_which = vec![];
            for item in &monkeys[i].items {
                let new_worry_level = reduce_worry_level_2(modulo, monkeys[i].op.eval(*item));
                if new_worry_level % monkeys[i].test_divisble_by == 0 {
                    what_to_which.push((new_worry_level, monkeys[i].throw_to_if_true));
                } else {
                    what_to_which.push((new_worry_level, monkeys[i].throw_to_if_false));
                }
            }

            monkeys[i].items = vec![];
            for (what, to_which) in what_to_which {
                monkeys[to_which].items.push(what);
            }
        }
    }

    let mut business = monkeys
        .iter()
        .map(|x| x.inspection_count)
        .collect::<Vec<usize>>();
    business.sort();
    business.reverse();
    println!("Day 11, part 1 {}", business[0] * business[1]);
}
