use crate::utils::read_lines;

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Robot {
    resource: Resource,
}

struct Blueprint {
    id: usize,
    robot_cost: HashMap<Resource, HashMap<Resource, i32>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Industry {
    robots: HashMap<Resource, i32>,
    resources: HashMap<Resource, i32>,
}

fn build(industry: &Industry, r: Resource, cost: &HashMap<Resource, i32>) -> Industry {
    let mut robots = industry.robots.clone();
    robots.entry(r).and_modify(|x| *x += 1);

    let mut resources = industry.resources.clone();
    for (r, amount) in cost {
        resources.entry(*r).and_modify(|x| *x -= amount);
    }

    Industry { robots, resources }
}

fn can_build(industry: &Industry, cost: &HashMap<Resource, i32>) -> bool {
    for (r, amount) in cost {
        if let Some(available_amount) = industry.resources.get(r) {
            if available_amount < amount {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn mine(industry: &Industry, _blueprint: &Blueprint) -> Industry {
    let mut resources = industry.resources.clone();
    for (resource, count) in &industry.robots {
        resources.entry(*resource).and_modify(|x| {
            *x += count;
        });
    }

    Industry {
        robots: industry.robots.clone(),
        resources,
    }
}

fn should_prune(industry: &Industry, b: &Blueprint, r: &Resource) -> bool {
    if r == &Resource::Geode {
        return false;
    }
    b.robot_cost
        .iter()
        .map(|(_, cost)| cost.get(r).unwrap_or(&0))
        .max()
        .unwrap()
        <= industry.robots.get(r).unwrap()
}

fn simulate(
    industry: Industry,
    blueprint: &Blueprint,
    time: i32,
    skipped: HashSet<Resource>,
) -> i32 {
    if time == 0 {
        let ret = *industry.resources.get(&Resource::Geode).unwrap();
        return ret;
    }

    let mut scores = vec![];

    let mut new_skipped: HashSet<Resource> = skipped.clone();
    for robot_type in [
        Resource::Ore,
        Resource::Clay,
        Resource::Obsidian,
        Resource::Geode,
    ] {
        if skipped.contains(&robot_type) {
            continue;
        }
        let robot_cost = &blueprint.robot_cost[&robot_type];
        if can_build(&industry, robot_cost) {
            new_skipped.insert(robot_type);
            if !should_prune(&industry, blueprint, &robot_type) {
                let industry_tmp = build(
                    &mine(&industry, blueprint),
                    robot_type,
                    blueprint.robot_cost.get(&robot_type).unwrap(),
                );
                scores.push(simulate(industry_tmp, blueprint, time - 1, HashSet::new()));
            }
        }
    }
    scores.push(simulate(
        mine(&industry, blueprint),
        blueprint,
        time - 1,
        new_skipped,
    ));

    *scores.iter().max().unwrap()
}

pub fn run() {
    let lines = read_lines("in/day19.in").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for (id, line) in lines.enumerate() {
        let l = line.unwrap();
        blueprints.push(scan!(&l;
            ("Blueprint ", let _id: u32, ": Each ore robot costs ", let ore_ore_cost: i32, "ore. Each clay robot costs ", let clay_ore_cost: i32, "ore. Each obsidian robot costs ", let obsidian_ore_cost: i32, " ore and ", let obsidian_clay_cost: i32, "clay. Each geode robot costs ", let geode_ore_cost: i32, "ore and ", let geode_obisdian_cost: i32, " obsidian.") => {
                Blueprint {
                    id: id+1,
                    robot_cost: HashMap::from_iter([
                        (Resource::Ore, HashMap::from_iter([(Resource::Ore, ore_ore_cost)])),
                    (Resource::Clay, HashMap::from_iter([(Resource::Ore, clay_ore_cost)])),
                    (Resource::Obsidian, HashMap::from_iter([(Resource::Ore, obsidian_ore_cost), (Resource::Clay, obsidian_clay_cost)])),
                    (Resource::Geode, HashMap::from_iter([(Resource::Ore, geode_ore_cost), (Resource::Obsidian, geode_obisdian_cost)])),
                    ])
                }
            },
        ).unwrap());
    }

    let start_industry = Industry {
        robots: HashMap::from_iter([
            (Resource::Ore, 1),
            (Resource::Clay, 0),
            (Resource::Obsidian, 0),
            (Resource::Geode, 0),
        ]),
        resources: HashMap::from_iter([
            (Resource::Ore, 0),
            (Resource::Clay, 0),
            (Resource::Obsidian, 0),
            (Resource::Geode, 0),
        ]),
    };

    println!(
        "Day 19, part 1: {}",
        blueprints
            .iter()
            .enumerate()
            .map(|(_, blueprint)| {
                simulate(start_industry.clone(), blueprint, 24, HashSet::new())
                    * blueprint.id as i32
            })
            .sum::<i32>()
    );
    println!(
        "Day 19, part 2: {}",
        blueprints
            .iter()
            .enumerate()
            .take(3)
            .map(|(_idx, blueprint)| {
                dbg!(simulate(
                    start_industry.clone(),
                    blueprint,
                    32,
                    HashSet::new()
                ))
            })
            .reduce(|x, y| x * y)
            .unwrap()
    );
}
