use std::collections::HashMap;

use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day12 {}

impl Solution for Day12 {
    fn name(&self) -> String {
        "Passage Pathing".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_12");
        let cavern = parse_input(&input);

        get_paths("start", &mut vec![], &cavern).to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_12");
        let cavern = parse_input(&input);

        get_paths2("start", &mut vec![], &cavern).to_string()
    }
}

fn parse_input(input: &String) -> HashMap<&str, Vec<&str>> {
    let mut cavern: HashMap<&str, Vec<&str>> = HashMap::new();
    let adjacency = input.lines()
        .map(|s| s.split('-').collect())
        .collect::<Vec<Vec<&str>>>();

    for item in adjacency {
        (*cavern.entry(item[0]).or_insert(vec![])).push(item[1]);
        (*cavern.entry(item[1]).or_insert(vec![])).push(item[0]);
    }

     cavern
}

fn get_paths<'a>(start: &'a str, visited: &mut Vec<&'a str>, cavern: &HashMap<&str, Vec<&'a str>>) -> u32 {
    let mut paths: u32 = 0;

    visited.push(start);

    if start == "end" {
        return 1;
    }

    for &cave in &cavern[start] {
        if cave == "start" || (cave.chars().all(char::is_lowercase) && visited.contains(&cave)) {
            continue;
        }

        paths += get_paths(cave, visited, cavern);
        visited.pop();
    }

    paths
}

fn get_paths2<'a>(start: &'a str, visited: &mut Vec<&'a str>, cavern: &HashMap<&str, Vec<&'a str>>) -> u32 {
    let mut paths: u32 = 0;

    visited.push(start);

    if start == "end" {
        return 1;
    }

    for &cave in &cavern[start] {
        if cave == "start" {
            continue;
        }

        let small_caves = visited.iter().map(|c| *c).filter(|&c| c.chars().all(char::is_lowercase)).collect::<Vec<&str>>();
        let duplicate = small_caves.len() != small_caves.iter().unique().count();
        let is_small_cave = cave.chars().all(char::is_lowercase);

        if duplicate && is_small_cave && visited.contains(&cave) {
            continue;
        }

        paths += get_paths2(cave, visited, cavern);
        visited.pop();
    }

    paths
}
