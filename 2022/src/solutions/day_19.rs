use std::cmp::max;

use aoc_common::{load, Solution};
use regex::Regex;

pub struct Day19 {}

impl Solution for Day19 {
    fn name(&self) -> String {
        "Not Enough Minerals".to_owned()
    }

    fn part_1(&self) -> String {
        let blueprints = parse_input(load("day_19"));
        let mut quality_level = 0;

        for blueprint in blueprints {
            let storage = Storage::new(blueprint.ore_cost);
            let robots_build = Storage::new(0);

            let most_geodes = get_max_geodes(&blueprint, 24 + 1, storage, robots_build, Construction::OreRobot);
            quality_level += blueprint.id * most_geodes;
        }

        quality_level.to_string()
    }

    fn part_2(&self) -> String {
        let blueprints = parse_input(load("day_19"));
        let mut result = 1;

        for blueprint in blueprints[0..3].iter() {
            let storage = Storage::new(blueprint.ore_cost);
            let robots_build = Storage::new(0);

            let most_geodes = get_max_geodes(&blueprint, 32 + 1, storage, robots_build, Construction::OreRobot);
            println!("{}", most_geodes);
            result *= most_geodes;

        }

        result.to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Construction {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodesRobot,
}

#[derive(Debug)]
struct Blueprint {
    id: isize,
    ore_cost: isize,
    clay_cost: isize,
    obsidian_cost: (isize, isize),
    geode_cost: (isize, isize),
    max_material_usage: Storage,
}

#[derive(Debug, Clone, Copy)]
struct Storage {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geodes: isize,
}

impl Storage {
    fn new(ore: isize) -> Self {
        Storage {
            ore,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

fn parse_input(input: String) -> Vec<Blueprint> {
    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let capture = regex.captures(line).unwrap();

        let blueprint_id = capture.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let ore_robot_cost = capture.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let clay_robot_cost = capture.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let obsidian_robot_cost = (
            capture.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            capture.get(5).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let geode_robot_cost = (
            capture.get(6).unwrap().as_str().parse::<isize>().unwrap(),
            capture.get(7).unwrap().as_str().parse::<isize>().unwrap(),
        );

        blueprints.push(Blueprint {
            id: blueprint_id,
            ore_cost: ore_robot_cost,
            clay_cost: clay_robot_cost,
            obsidian_cost: obsidian_robot_cost,
            geode_cost: geode_robot_cost,
            max_material_usage: Storage {
                ore: max(
                    ore_robot_cost,
                    max(
                        clay_robot_cost,
                        max(obsidian_robot_cost.0, geode_robot_cost.0),
                    ),
                ),
                clay: obsidian_robot_cost.1,
                obsidian: geode_robot_cost.1,
                geodes: 0,
            },
        });
    }

    blueprints
}

fn get_max_geodes(
    blueprint: &Blueprint,
    mut turns_left: isize,
    mut storage: Storage,
    mut robots_build: Storage,
    to_construct: Construction,
) -> isize {
    let mut robot_constructed = false;
    while !robot_constructed && turns_left > 0 {
        match to_construct {
            Construction::OreRobot => {
                if storage.ore >= blueprint.ore_cost {
                    storage.ore -= blueprint.ore_cost;
                    robot_constructed = true;
                }
            }
            Construction::ClayRobot => {
                if storage.ore >= blueprint.clay_cost {
                    storage.ore -= blueprint.clay_cost;
                    robot_constructed = true;
                }
            }
            Construction::ObsidianRobot => {
                if storage.ore >= blueprint.obsidian_cost.0 && storage.clay >= blueprint.obsidian_cost.1 {
                    storage.ore -= blueprint.obsidian_cost.0;
                    storage.clay -= blueprint.obsidian_cost.1;
                    robot_constructed = true;
                }
            }
            Construction::GeodesRobot => {
                if storage.ore >= blueprint.geode_cost.0 && storage.obsidian >= blueprint.geode_cost.1 {
                    storage.ore -= blueprint.obsidian_cost.0;
                    storage.obsidian -= blueprint.geode_cost.1;
                    robot_constructed = true;
                }
            }
        }

        storage.ore += robots_build.ore;
        storage.clay += robots_build.clay;
        storage.obsidian += robots_build.obsidian;
        storage.geodes += robots_build.geodes;
        turns_left -= 1;

        if robot_constructed {
            match to_construct {
                Construction::OreRobot => { robots_build.ore += 1 },
                Construction::ClayRobot => { robots_build.clay += 1 },
                Construction::ObsidianRobot => { robots_build.obsidian += 1 },
                Construction::GeodesRobot => { robots_build.geodes += 1 },
            }
        }
    }

    let mut max_geodes = storage.geodes;
    if turns_left > 0 {
        for next_robot in [Construction::OreRobot, Construction::ClayRobot, Construction::ObsidianRobot, Construction::GeodesRobot] {
            if next_robot == Construction::ObsidianRobot && robots_build.clay == 0 {
                continue;
            }

            if next_robot == Construction::GeodesRobot && robots_build.obsidian == 0 {
                continue;
            }

            // Stops building unnessasary robots 
            if next_robot == Construction::OreRobot && robots_build.ore == blueprint.max_material_usage.ore ||
                next_robot == Construction::ClayRobot && robots_build.clay == blueprint.obsidian_cost.1 ||
                next_robot == Construction::ObsidianRobot && robots_build.obsidian == blueprint.geode_cost.1 
            {
                continue;
            }

            let geodes = get_max_geodes(blueprint, turns_left, storage, robots_build, next_robot);
            max_geodes = max(max_geodes, geodes);
        }
    }

    max_geodes
}