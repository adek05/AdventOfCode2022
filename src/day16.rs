use crate::utils::read_lines;
use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Graph = HashMap<String, Vec<String>>;

fn get_key(set: &HashSet<String>) -> String {
    let mut valves: Vec<String> = set.iter().cloned().collect();
    valves.push("".to_string());
    valves.sort();
    valves.join(".")
}

pub fn bfs(start_valve: &String, tunnels: &Graph) -> HashMap<(String, String), i32> {
    let mut queue = VecDeque::new();
    queue.push_back((start_valve.clone(), 0));

    let mut res: HashMap<(String, String), i32> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    while let Some((next_valve, dist)) = queue.pop_front() {
        if visited.contains(&next_valve) {
            continue;
        }
        visited.insert(next_valve.clone());
        if &next_valve != start_valve {
            res.insert((start_valve.clone(), next_valve.clone()), dist);
        }
        for next in tunnels.get(&next_valve).unwrap_or(&vec![]) {
            queue.push_back((next.clone(), dist + 1));
        }
    }
    res
}

pub fn part1(
    max: &mut i32,
    valve: &String,
    score: i32,
    open: &mut HashSet<String>,
    time_left: i32,
    tunnels: &Graph,
    pressures: &HashMap<String, i32>,
    distances: &HashMap<(String, String), i32>,
    recurse: bool,
    memory: &mut HashMap<String, i32>,
) {
    if open.iter().position(|x| x == valve).is_some() && valve != &"AA".to_string() {
        return;
    }
    let new_score = score
        + std::cmp::max(time_left - (if valve != "AA" { 1 } else { 0 }), 0)
            * pressures.get(valve).unwrap();

    let mut delta = 0;
    open.insert(valve.clone());
    if recurse {
        let mut new_open = open.clone();
        let key = get_key(open);
        if memory.contains_key(&key) {
            delta = *memory.get(&key).unwrap();
        } else {
            delta = run_part1(
                26,
                &mut new_open,
                &tunnels,
                &pressures,
                &distances,
                false,
                memory,
            );
            memory.insert(key, delta);
        }
    }

    *max = std::cmp::max(new_score + delta, *max);
    if time_left < 0 {
        open.remove(valve);
        return;
    }

    for next_valve in tunnels.keys() {
        if let Some(next_valve_dist) = distances.get(&(valve.clone(), next_valve.clone())) {
            let new_time_left = time_left - (if valve != "AA" { 1 } else { 0 }) - next_valve_dist;
            part1(
                max,
                next_valve,
                new_score,
                open,
                new_time_left,
                tunnels,
                pressures,
                distances,
                recurse,
                memory,
            );
        }
    }
    open.remove(valve);
}

pub fn run_part1(
    time_left: i32,
    visited: &mut HashSet<String>,
    tunnels: &Graph,
    pressures: &HashMap<String, i32>,
    distances: &HashMap<(String, String), i32>,
    recurse: bool,
    memory: &mut HashMap<String, i32>,
) -> i32 {
    let mut max: i32 = 0;
    part1(
        &mut max,
        &"AA".to_string(),
        0,
        visited,
        time_left,
        &tunnels,
        &pressures,
        &distances,
        recurse,
        memory,
    );
    max
}

pub fn run() {
    let lines = read_lines("in/day16.in").unwrap();
    // let lines = read_lines("in/day16small.in").unwrap();

    let mut valve_pressure: HashMap<String, i32> = HashMap::new();
    let mut tunnels: Graph = HashMap::new();

    for line in lines {
        let l = line.unwrap();
        scan!(&l;
            ("Valve ", let source_valve: Word<String>, " has flow rate=", let pressure: i32, "; tunnels lead to valves ", [let target_valves: Word<String>],+: Vec<String>) => {
                valve_pressure.insert(source_valve.clone(), pressure);
                tunnels.entry(source_valve.clone()).and_modify(|t|
                    t.extend(target_valves.clone())
                ).or_insert(target_valves.clone());
            },
            ("Valve ", let source_valve: Word<String>, " has flow rate=", let pressure: i32, "; tunnel leads to valve ", [let target_valves: Word<String>],+: Vec<String>) => {
                valve_pressure.insert(source_valve.clone(), pressure);
                tunnels.entry(source_valve.clone()).and_modify(|t|
                    t.extend(target_valves.clone())
                ).or_insert(target_valves.clone());
            }
        ).unwrap();
    }

    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    for valve in tunnels.keys() {
        distances.extend(bfs(valve, &tunnels));
    }
    distances = distances
        .into_iter()
        .filter(|((_, dest), _)| valve_pressure.get(dest).unwrap() > &0)
        .collect();

    {
        let mut visited: HashSet<String> = HashSet::new();
        let mut memory: HashMap<String, i32> = HashMap::new();
        let max: i32 = run_part1(
            30,
            &mut visited,
            &tunnels,
            &valve_pressure,
            &distances,
            false,
            &mut memory,
        );
        println!("Day 16, part 1 {}", max);
    }

    {
        let mut visited: HashSet<String> = HashSet::new();
        let mut memory: HashMap<String, i32> = HashMap::new();
        let max: i32 = run_part1(
            26,
            &mut visited,
            &tunnels,
            &valve_pressure,
            &distances,
            true,
            &mut memory,
        );
        println!("Day 16, part 2 {}", max);
    }
}
