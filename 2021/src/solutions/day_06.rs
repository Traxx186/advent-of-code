use std::collections::HashMap;
use aoc_common::{load, Solution};

pub struct Day06 {}

impl Solution for Day06 {

    fn name(&self) -> String {
        "Lanternfish".to_owned()
    }

    fn part_1(&self) -> String {
        let data = Lanterfish::parse_input(load("day_06"));
        let amount_of_fish= Lanterfish::simulate(data, 80);

        amount_of_fish.to_string()
    }

    fn part_2(&self) -> String {
        let data = Lanterfish::parse_input(load("day_06"));
        let amount_of_fish= Lanterfish::simulate(data, 256);

        amount_of_fish.to_string()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Lanterfish {
    timer: u32
}

impl Lanterfish {

    fn new(timer: u32) -> Lanterfish {
        Lanterfish { timer }
    }

    fn parse_input(input: String) -> Vec<Lanterfish> {
        input.split(',')
            .map(|x| Lanterfish::new(x.parse().unwrap()))
            .collect()
    }

    fn simulate(input: Vec<Lanterfish>, days: u32) -> usize {
        let mut fish = HashMap::new();

        for i in input {
            *fish.entry(i).or_insert(0) += 1;
        }

        for _ in 0..days {
            let mut new_fish = HashMap::new();
            for i in &fish {
                if i.0.timer == 0 {
                    *new_fish.entry(Lanterfish::new(6)).or_insert(0) += *i.1;
                    *new_fish.entry(Lanterfish::new(8)).or_insert(0) += *i.1;
                    continue;
                }

                *new_fish.entry(Lanterfish::new(i.0.timer - 1)).or_insert(0) += *i.1;
            }

            fish = new_fish;
        }

        fish.values().sum()
    }

}