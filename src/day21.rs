use crate::utils::read_lines;

use scan_rules::scanner::Word;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Expression {
    Variable(String),
    Literal(i64),
    Human,
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Cmp(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn is_number(&self) -> bool {
        match self {
            Expression::Literal(_) => true,
            _ => false,
        }
    }

    pub fn get_number(&self) -> i64 {
        match self {
            Expression::Literal(x) => *x,
            _ => panic!("Not a number"),
        }
    }

    pub fn can_eval(&self) -> bool {
        match self {
            Expression::Variable(_) => false,
            Expression::Cmp(_, _) => false,
            Expression::Literal(_) => true,
            Expression::Add(e1, e2) => e1.can_eval() && e2.can_eval(),
            Expression::Sub(e1, e2) => e1.can_eval() && e2.can_eval(),
            Expression::Mul(e1, e2) => e1.can_eval() && e2.can_eval(),
            Expression::Div(e1, e2) => e1.can_eval() && e2.can_eval(),
            Expression::Human => false,
        }
    }

    pub fn eval(&self) -> i64 {
        match self {
            Expression::Variable(_) => panic!("Should not try to eval Variable"),
            Expression::Literal(val) => *val,
            Expression::Cmp(_, _) => panic!("Should not try to eval that"),
            Expression::Add(e1, e2) => e1.eval() + e2.eval(),
            Expression::Sub(e1, e2) => e1.eval() - e2.eval(),
            Expression::Mul(e1, e2) => e1.eval() * e2.eval(),
            Expression::Div(e1, e2) => e1.eval() / e2.eval(),
            Expression::Human => panic!("Should not try to eval Human"),
        }
    }

    pub fn substitute(&self, variable: &String, value: i64) -> Expression {
        match self {
            Expression::Variable(v) if variable == v => Expression::Literal(value),
            e @ Expression::Variable(_) => e.clone(),
            e @ Expression::Literal(_) => e.clone(),
            Expression::Add(e1, e2) => Expression::Add(
                Box::new(e1.substitute(variable, value)),
                Box::new(e2.substitute(variable, value)),
            ),
            Expression::Sub(e1, e2) => Expression::Sub(
                Box::new(e1.substitute(variable, value)),
                Box::new(e2.substitute(variable, value)),
            ),
            Expression::Mul(e1, e2) => Expression::Mul(
                Box::new(e1.substitute(variable, value)),
                Box::new(e2.substitute(variable, value)),
            ),
            Expression::Div(e1, e2) => Expression::Div(
                Box::new(e1.substitute(variable, value)),
                Box::new(e2.substitute(variable, value)),
            ),
            Expression::Cmp(e1, e2) => Expression::Cmp(
                Box::new(e1.substitute(variable, value)),
                Box::new(e2.substitute(variable, value)),
            ),
            Expression::Human => Expression::Human,
        }
    }

    pub fn backpropagate(&self, target_value: i64) -> i64 {
        match self {
            Expression::Human => target_value,
            Expression::Add(x, e) if x.is_number() => e.backpropagate(target_value - x.get_number()),
            Expression::Add(e, x) if x.is_number() => e.backpropagate(target_value - x.get_number()),
            Expression::Sub(x, e) if x.is_number()  => e.backpropagate(x.get_number() - target_value),
            Expression::Sub(e, x) if x.is_number() => e.backpropagate(target_value + x.get_number()),
            Expression::Mul(x, e) if x.is_number()  => e.backpropagate(target_value / x.get_number()),
            Expression::Mul(e, x) if x.is_number() => e.backpropagate(target_value / x.get_number()),
            // t = x/e => e = x/t
            Expression::Div(x, e) if x.is_number()  => e.backpropagate(x.get_number() / target_value),
            // t = e/x => e = t*x
            Expression::Div(e, x) if x.is_number() => e.backpropagate(x.get_number() * target_value),
            e => panic!("{:?}", e),

        }
    }
}

pub fn run() {
    let lines = read_lines("in/day21small.in").unwrap();

    let mut operations: HashMap<String, Expression> = HashMap::new();
    let mut to_substitute: Vec<(String, i64)> = vec![];

    for line in lines {
        let l = line.unwrap();
        scan!(
            &l;
            ("root: ", let input1: Word<String>, " + ", let input2: Word<String>) => {
                operations.insert(
                "root".to_string(),
                Expression::Cmp(
                    Box::new(Expression::Variable(input1)),
                    Box::new(Expression::Variable(input2)),
                ));
            },
            ("humn: ", let _: i64) => {
                operations.insert(
                "hmn".to_string(),
                Expression::Human,
                );
            },
            (let monkey: Word<String>, ":", let value: i64) => {
                operations.insert(monkey.clone(), Expression::Literal(value));
                to_substitute.push((monkey, value));
            },
            (let monkey: Word<String>, ": ", let input1: Word<String>, " + ", let input2: Word<String>) => {
                operations.insert(monkey,
                 Expression::Add(Box::new(Expression::Variable(input1)), Box::new(Expression::Variable(input2))));
            },
            (let monkey: Word<String>, ": ", let input1: Word<String>, " - ", let input2: Word<String>) => {
                operations.insert(monkey,
                 Expression::Sub(Box::new(Expression::Variable(input1)), Box::new(Expression::Variable(input2))));
            },
            (let monkey: Word<String>, ": ", let input1: Word<String>, " * ", let input2: Word<String>) => {
                operations.insert(monkey,
                 Expression::Mul(Box::new(Expression::Variable(input1)), Box::new(Expression::Variable(input2))));
            },
            (let monkey: Word<String>, ": ", let input1: Word<String>, " / ", let input2: Word<String>) => {
                operations.insert(monkey,
                 Expression::Div(Box::new(Expression::Variable(input1)), Box::new(Expression::Variable(input2))));
            }
        ).unwrap();
    }

    while !operations.get(&"root".to_string()).unwrap().is_number() {
        for (var, val) in &to_substitute {
            operations = operations.iter().map(|(v, e)| (v.clone(), e.substitute(&var, *val))).collect();
        }
        to_substitute = vec![];

        operations = operations.iter().map(|(variable, e)| {
            if e.can_eval() && !e.is_number() {
                let res = e.eval();
                to_substitute.push((variable.clone(), res));
                return (variable.clone(), Expression::Literal(res));
            } else {
                return (variable.clone(), e.clone());
            }
        }).collect();
        dbg!(
            operations.get(&"root".to_string()).unwrap()
        );

        if let Expression::Cmp(e1, e2) = operations.get(&"root".to_string()).unwrap() {
            if e1.is_number() && to_substitute.is_empty() {
                dbg!(e2.backpropagate(e1.get_number()));
            }
            if e2.is_number() && to_substitute.is_empty() {
                dbg!(e1.backpropagate(e2.get_number()));
            }
        }
    }

    // if let Expression::Literal(v) = operations.get(&"root".to_string()).unwrap() {
    //     println!("Day 21, part 1: {}", v);
    // }
}
