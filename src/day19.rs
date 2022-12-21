use crate::utils::read_lines;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    ore_robot: HashMap<Resource, i32>,
    clay_robot: HashMap<Resource, i32>,
    obsidian_robot: HashMap<Resource, i32>,
    geode_robot: HashMap<Resource, i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Industry {
    robots: HashMap<Resource, i32>,
    resources: HashMap<Resource, i32>,
}

impl Hash for Industry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let x = format!(
            "{}-{}-{}-{}-{}-{}-{}-{}",
            self.robots.get(&Resource::Ore).unwrap(),
            self.robots.get(&Resource::Clay).unwrap(),
            self.robots.get(&Resource::Obsidian).unwrap(),
            self.robots.get(&Resource::Geode).unwrap(),
            self.resources.get(&Resource::Ore).unwrap(),
            self.resources.get(&Resource::Clay).unwrap(),
            self.resources.get(&Resource::Obsidian).unwrap(),
            self.resources.get(&Resource::Geode).unwrap(),
        );
        x.hash(state);
    }
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
        if let Some(available_amount) = industry.resources.get(&r) {
            if available_amount < &amount {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn mine(industry: &Industry) -> Industry {
    let mut resources = industry.resources.clone();
    for (resource, count) in &industry.robots {
        resources.entry(*resource).and_modify(|x| *x += count);
    }

    Industry {
        robots: industry.robots.clone(),
        resources,
    }
}

fn simulate(
    industry: Industry,
    blueprint: &Blueprint,
    time: i32,
    cache: &mut HashMap<Industry, i32>,
) -> i32 {
    if let Some(res) = cache.get(&industry) {
        return *res;
    }
    assert!(industry.robots.len() < 31);
    if time == 0 {
        let ret = *industry.resources.get(&Resource::Geode).unwrap();
        return ret;
    }

    let mut scores = vec![];

    if can_build(&industry, &blueprint.ore_robot) {
        if industry.robots.get(&Resource::Ore).unwrap() < &10 {
            let industry_tmp = build(&mine(&industry), Resource::Ore, &blueprint.ore_robot);
            scores.push(simulate(industry_tmp, blueprint, time - 1, cache));
        }
    }
    if can_build(&industry, &blueprint.clay_robot) {
        if industry.robots.get(&Resource::Clay).unwrap() < &10 {
            let industry_tmp = build(&mine(&industry), Resource::Clay, &blueprint.clay_robot);
            scores.push(simulate(industry_tmp, blueprint, time - 1, cache));
        }
    }
    if can_build(&industry, &blueprint.obsidian_robot) {
        if industry.robots.get(&Resource::Obsidian).unwrap() < &10 {
            let industry_tmp = build(
                &mine(&industry),
                Resource::Obsidian,
                &blueprint.obsidian_robot,
            );
            scores.push(simulate(industry_tmp, blueprint, time - 1, cache));
        }
    }
    if can_build(&industry, &blueprint.geode_robot) {
        let industry_tmp = build(&mine(&industry), Resource::Geode, &blueprint.geode_robot);
        scores.push(simulate(industry_tmp, blueprint, time - 1, cache));
    }
    scores.push(simulate(mine(&industry), blueprint, time - 1, cache));

    let best = *scores.iter().max().unwrap();
    cache.insert(industry.clone(), best);

    best
}

pub fn run() {
    let lines = read_lines("in/day19small.in").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in lines {
        let l = line.unwrap();
        blueprints.push(scan!(&l;
            ("Blueprint ", let _id: u32, ": Each ore robot costs ", let ore_ore_cost: i32, "ore. Each clay robot costs ", let clay_ore_cost: i32, "ore. Each obsidian robot costs ", let obsidian_ore_cost: i32, " ore and ", let Obsidian_clay_cost: i32, "clay. Each geode robot costs ", let geode_ore_cost: i32, "ore and ", let geode_obisdian_cost: i32, " obsidian.") => {
                Blueprint {
                    ore_robot: HashMap::from_iter([(Resource::Ore, ore_ore_cost)]),
                    clay_robot: HashMap::from_iter([(Resource::Ore, clay_ore_cost)]),
                    obsidian_robot: HashMap::from_iter([(Resource::Ore, obsidian_ore_cost), (Resource::Clay, Obsidian_clay_cost)]),
                    geode_robot: HashMap::from_iter([(Resource::Ore, geode_ore_cost), (Resource::Obsidian, geode_obisdian_cost)]),
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
    for blueprint in &blueprints {
        let mut lookup: HashMap<Industry, i32> = HashMap::new();
        println!(
            "{}",
            simulate(start_industry.clone(), &blueprint, 21, &mut lookup)
        );
    }
}
