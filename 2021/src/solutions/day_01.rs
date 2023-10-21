use aoc_common::{load, Solution};

pub struct Day01 {}

impl Solution for Day01 {

    fn name(&self) -> String {
        "Sonar Sweep".to_owned()
    }

    fn part_1(&self) -> String {
        let measurements = load("day_01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let increases = measurements
            .windows(2)
            .filter(|x| x[0] < x[1])
            .count();

        increases.to_string()
    }

    fn part_2(&self) -> String {
        let measurements = load("day_01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut increases = 0;
        for i in 3..measurements.len() {
            let a = measurements[i - 1] + measurements[i - 2] + measurements[i - 3];
            let b = measurements[i] + measurements[i - 1] + measurements[i - 2];

            if b > a { increases += 1; }
        }

        increases.to_string()
    }
}