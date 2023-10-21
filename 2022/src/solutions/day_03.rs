use aoc_common::{load, Solution};
use itertools::Itertools;
use regex::Regex;

pub struct Day03 {}

impl Solution for Day03 {

    fn name(&self) -> String {
        "Rucksack Reorganization".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_03");

        let total: isize = input.lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|compartment| find_occurring_items(&compartment.0.to_string(), &compartment.1.to_string()))
            .map(|items| get_char_value(items.chars().collect_vec()[0]))
            .sum();

       total.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_03");
        let total: isize = input.lines()
            .chunks(3)
            .into_iter()
            .map(|mut group| find_group_badge(&group.next().unwrap(), &group.next().unwrap(), &&group.next().unwrap()))
            .map(|items| get_char_value(items.chars().collect_vec()[0]))
            .sum();

        total.to_string()
    }
}

fn get_char_value(input: char) -> isize {
    if input.is_uppercase() {
        (input as isize) - 64 + 26
    } else {
        (input as isize) - 96
    }
}

fn find_occurring_items(a: &String, b: &String) -> String {
    let regex = Regex::new(format!("[{}]", a).as_str()).unwrap();
    let matching_item = regex.find_iter(b).map(|el| el.as_str());

    String::from_iter(matching_item.collect_vec())
}

fn find_group_badge(bag_one: &&str, bag_two: &&str, bag_three: &&str) -> String {
    let a = find_occurring_items(&bag_one.to_string(), &bag_two.to_string());

    find_occurring_items(&a, &bag_three.to_string())
}