use std::collections::HashMap;

use aoc_common::{Solution, load};

pub struct Day01;

impl Solution for Day01 {

    fn name(&self) -> String {
        "Trebuchet?!".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_01");
        let mut calibration_values: Vec<isize> = Vec::new();

        for line in input.lines() {
            let first_number = line.chars()
                .find(|char| char.is_ascii_digit())
                .map(|char| char as isize - 48)
                .unwrap_or(0);

            let last_number = line.chars().rev()
                .find(|char| char.is_ascii_digit())
                .map(|char| char as isize - 48)
                .unwrap_or(0);

            calibration_values.push(first_number * 10 + last_number);
        }

        let sum: isize = calibration_values.iter().sum();
        sum.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_01");
        let mut calibration_values: Vec<isize> = Vec::new();

        for line in input.lines() {
            let first_number = find_first_number(line);
            let last_number = find_last_number(line);

            calibration_values.push(first_number * 10 + last_number);
        }

        let sum: isize = calibration_values.iter().sum();
        sum.to_string()
    }
}

fn find_first_number(line: &str) -> isize {
    let text_number: HashMap<&str, isize> = numbers_map();
    let mut searched_string = String::new();

    for char in line.chars() {
        if char.is_ascii_digit() {
            return char as isize - 48;
        }

        searched_string.push(char);
        for (key, number) in &text_number {
            if searched_string.contains(key) {
                return number.to_owned();
            }
        }
    }

    return 0;
}

fn find_last_number(line: &str) -> isize {
    let text_number: HashMap<&str, isize> = numbers_map();
    let mut searched_string = String::new();

    for char in line.chars().rev() {
        if char.is_ascii_digit() {
            return char as isize - 48;
        }

        searched_string.insert(0, char);
        for (key, number) in &text_number {
            if searched_string.contains(key) {
                return number.to_owned();
            }
        }
    }

    return 0;

}

fn numbers_map() -> HashMap<&'static str, isize> {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8), 
        ("nine", 9),
    ])
}