use crate::utils::read_lines;

#[derive(Debug, Clone)]
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
    level / 3
}

pub fn reduce_worry_level_2(modulo: u64, level: u64) -> u64 {
    level % modulo
}

#[derive(Clone, Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub op: Op,
    pub test_divisble_by: u64,
    pub throw_to_if_true: usize,
    pub throw_to_if_false: usize,
    pub inspection_count: usize,
}

fn simulate(
    mut monkeys: Vec<Monkey>,
    reduce_worry_level: Box<dyn Fn(u64) -> u64>,
    iter_count: u32,
) -> usize {
    for _ in 0..iter_count {
        for i in 0..monkeys.len() {
            monkeys[i].inspection_count += monkeys[i].items.len();

            let mut what_to_which = vec![];
            for item in &monkeys[i].items {
                let new_worry_level = reduce_worry_level(monkeys[i].op.eval(*item));
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
    business[0] * business[1]
}

pub fn run() {
    let lines = read_lines("in/day11.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let mut modulo = 1;
    let monkeys: Vec<Monkey> = lines.split(|l| l.is_empty()).into_iter().map(|monkey_lines| {
        let mut monkey_line = monkey_lines.iter();
        monkey_line.next();
        let items =
        scan!(monkey_line.next().unwrap(); ("  Starting items: ", [let items: u64],+: Vec<u64>) => items ).unwrap();
        let op =
        scan!(monkey_line.next().unwrap();
            ("  Operation: new = old + ", let delta: u64) => Op::Add(delta),
            ("  Operation: new = old * old") => Op::Square,
            ("  Operation: new = old * ", let delta: u64) => Op::Mul(delta) 
        ).unwrap();
        let test_divisble_by = scan!(monkey_line.next().unwrap();
            ("  Test: divisible by ", let divider: u64) => divider
        ).unwrap();
        let throw_to_if_true = scan!(monkey_line.next().unwrap();
            ("    If true: throw to monkey ", let monkey: usize) => monkey
        ).unwrap();
        let throw_to_if_false = scan!(monkey_line.next().unwrap();
            ("    If false: throw to monkey ", let monkey: usize) => monkey
        ).unwrap();
        modulo *= test_divisble_by;

        Monkey { items, op, test_divisble_by, throw_to_if_true, throw_to_if_false , inspection_count: 0}
    }).collect();

    println!(
        "Day 11, part 1 {}",
        simulate(monkeys.clone(), Box::new(reduce_worry_level), 20)
    );
    println!(
        "Day 11, part 2 {}",
        simulate(monkeys, Box::new(move |x| x % modulo), 10_000)
    );
}
