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

        for line in input.lines() {
            let numbers = regex.find_iter(line)
                .map(|f| f.as_str())
                .collect_vec();

            let first_and_last = numbers.first().unwrap().to_string().push_str(numbers.last().unwrap());
        }        

        "".to_owned()
    }

    fn part_2(&self) -> String {
        todo!()
    }
}

