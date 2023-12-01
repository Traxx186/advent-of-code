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
        let regex = Regex::new(r"\d").unwrap();
        let mut calibration_values: Vec<isize> = Vec::new();

        for line in input.lines() {
            let numbers = regex.find_iter(line)
                .map(|f| f.as_str())
                .collect_vec();

            let mut first_numer = numbers.first().unwrap().to_string();
            let second_numer = numbers.last().unwrap();

            first_numer.push_str(second_numer);
            calibration_values.push(first_numer.parse::<isize>().unwrap())
        }        

        let sum: isize = calibration_values.iter()
            .sum();
        
        sum.to_string()
    }

    fn part_2(&self) -> String {
        todo!()
    }
}

