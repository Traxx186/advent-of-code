use std::collections::HashMap;

use aoc_common::{Solution, load};
use itertools::Itertools;
use regex::Regex;

pub struct Day01;

impl Solution for Day01 {

    fn name(&self) -> String {
        "Trebuchet?!".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_01");
        let mut calibration_values: Vec<isize> = Vec::new();

        for line in input.lines() {
            let first_last_number = find_first_last_number(line);
            calibration_values.push(first_last_number.parse::<isize>().unwrap())
        }        

        let sum: isize = calibration_values.iter().sum();
        sum.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_01");
        let mut calibration_values: Vec<isize> = Vec::new();

        for line in input.lines() {
            let replaced_text_numbers = text_to_number(line);
            let first_last_number = find_first_last_number(line);
            calibration_values.push(first_last_number.parse::<isize>().unwrap())
        }

        let sum: isize = calibration_values.iter().sum();
        sum.to_string()
    }
}

fn find_first_last_number(input: &str) -> String {
    let regex = Regex::new(r"\d").unwrap();
    let numbers = regex.find_iter(line)
        .map(|f| f.as_str())
        .collect_vec();

    let mut first_numer = numbers.first().unwrap().to_string();
    let second_numer = numbers.last().unwrap();

    first_numer.push_str(second_numer);
    first_numer
}

fn text_to_number(&mut input: &str) -> String {
    let text_number = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8), 
        ("nine", 9),
    ]);

    for (key, value) in text_number.into_iter() {
        input.replace(key, value);
    }
}