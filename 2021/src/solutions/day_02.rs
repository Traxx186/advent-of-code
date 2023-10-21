use aoc_common::{load, Solution};

pub struct Day02 {}

impl Solution for Day02 {

    fn name(&self) -> String {
        "Dive!".to_owned()
    }

    fn part_1(&self) -> String {
        let inputs = load("day_02");
        let mut forward: u32 = 0;
        let mut depth: u32 = 0;

        for input in inputs.lines() {
            let command = input.split(' ').collect::<Vec<&str>>();
            let command_value = command[1].parse::<u32>().unwrap();
            
            match command[0] {
                "forward" => forward += command_value,
                "down" => depth += command_value,
                "up" => depth -= command_value,
                _ => panic!("Command doesnt exist!")
            }
        }

        (forward * depth).to_string()
    }

    fn part_2(&self) -> String {
        let inputs = load("day_02");
        let mut forward: u32 = 0;
        let mut aim: u32 = 0;
        let mut depth: u32 = 0;

        for input in inputs.lines() {
            let command = input.split(' ').collect::<Vec<&str>>();
            let command_value = command[1].parse::<u32>().unwrap();

            match command[0] {
                "forward" => {
                    forward += command_value;
                    depth += command_value * aim;
                },
                "down" => aim += command_value,
                "up" => aim -= command_value,
                _ => panic!("Command doesnt exist!")
            }
        }

        (forward * depth).to_string()
    }
}