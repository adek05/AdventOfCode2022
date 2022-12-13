use crate::utils::read_lines;
use std::cmp::Ordering;
use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Number(i32),
    List(Vec<Value>),
}

fn parse_number(it: &mut Peekable<Chars>) -> Value {
    let mut number_input = String::new();
    while let Some(next_char) = it.peek() {
        if next_char.is_ascii_digit() {
            number_input.push(it.next().unwrap());
        } else {
            assert!(
                next_char == &',' || next_char == &']',
                "Expected comma or end of list"
            );
            break;
        }
    }
    Value::Number(number_input.parse().unwrap())
}

fn parse_list(it: &mut Peekable<Chars>) -> Value {
    let mut vals = vec![];
    while let Some(next_char) = it.peek() {
        if next_char == &']' {
            it.next().unwrap();
            break;
        }
        if next_char == &',' {
            it.next().unwrap();
        }
        vals.push(parse_value(it));
    }
    Value::List(vals)
}

fn parse_value(it: &mut Peekable<Chars>) -> Value {
    if let Some(next_char) = it.peek() {
        if next_char == &'[' {
            it.next().unwrap();
            parse_list(it)
        } else {
            parse_number(it)
        }
    } else {
        panic!("Should not happend");
    }
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(a_number), Value::Number(b_number)) => a_number.cmp(b_number),
        (Value::List(a_list), Value::List(b_list)) => {
            for i in 0..a_list.len() {
                if let Some(a_elem) = a_list.get(i) {
                    if let Some(b_elem) = b_list.get(i) {
                        let cmp_res = compare(a_elem, b_elem);
                        if cmp_res == Ordering::Equal {
                            continue;
                        } else {
                            return cmp_res;
                        }
                    } else {
                        return Ordering::Greater;
                    }
                } else {
                    return a_list.len().cmp(&b_list.len());
                }
            }
            a_list.len().cmp(&b_list.len())
        }
        (a @ Value::Number(_), b) => compare(&Value::List(vec![a.clone()]), b),
        (a, b @ Value::Number(_)) => compare(a, &Value::List(vec![b.clone()])),
    }
}

pub fn run() {
    let lines = read_lines("in/day13.in").unwrap();

    let pairs: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut idx = 1;
    let mut res = 0;

    let mut all_packets = vec![];
    for pair in pairs.split(|x| x.is_empty()) {
        let first = parse_value(&mut pair[0].chars().peekable());
        let second = parse_value(&mut pair[1].chars().peekable());
        all_packets.push(first.clone());
        all_packets.push(second.clone());

        if compare(&first, &second) == Ordering::Less {
            res += idx;
        }
        idx += 1;
    }
    println!("Day 13, part 1 {}", res);

    let divider_1 = Value::List(vec![Value::List(vec![Value::Number(2)])]);
    let divider_2 = Value::List(vec![Value::List(vec![Value::Number(6)])]);
    all_packets.push(divider_1.clone());
    all_packets.push(divider_2.clone());

    all_packets.sort_by(compare);

    println!(
        "Day 13, part 1 {}",
        (all_packets.iter().position(|x| x == &divider_1).unwrap() + 1)
            * (all_packets.iter().position(|x| x == &divider_2).unwrap() + 1)
    );
}
