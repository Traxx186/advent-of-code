use aoc_common::{Solution, load};
use itertools::Itertools;
use regex::Regex;

pub struct Day11 {}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    opertation: Operation,
    divisor: u64,
    throw_to: (usize, usize),
    inspections: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Solution for Day11 {

    fn name(&self) -> String {
        "Monkey in the Middle".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_11");
        let mut monkeys = input.split("\n\n")
            .map(|line| parse_monkey(line))
            .collect_vec();

        for _ in 0..20  {
            for i in 0..monkeys.len()  {
                let (next_if_true, next_if_false) = monkeys[i].throw_to;
                let new = monkeys[i].items
                    .iter()
                    .map(|item| monkeys[i].opertation.apply(*item) / 3)
                    .map(|x| (x % monkeys[i].divisor == 0, x))
                    .collect_vec();

                monkeys[next_if_true].items
                    .extend(
                        new.iter()
                            .filter_map(|x| if x.0 { Some(x.1) } else { None }),
                    );

                monkeys[next_if_false].items
                    .extend(
                        new.iter()
                            .filter_map(|x| if !x.0 { Some(x.1) } else { None }),
                    );

                monkeys[i].inspections += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }

        let inspections_list: (usize, usize) = monkeys.into_iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .unwrap();

        (inspections_list.0 * inspections_list.1).to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_11");
        let mut monkeys = input.split("\n\n")
            .map(|line| parse_monkey(line))
            .collect_vec();

        let filter = monkeys.iter().map(|m| m.divisor).product::<u64>();

        for _ in 0..10_000  {
            for i in 0..monkeys.len()  {
                let (next_if_true, next_if_false) = monkeys[i].throw_to;
                let new = monkeys[i].items
                    .iter()
                    .map(|item| monkeys[i].opertation.apply(*item))
                    .map(|x| (x % monkeys[i].divisor == 0, x % filter))
                    .collect_vec();

                monkeys[next_if_true].items
                    .extend(
                        new.iter()
                            .filter_map(|x| if x.0 { Some(x.1) } else { None }),
                    );

                monkeys[next_if_false].items
                    .extend(
                        new.iter()
                            .filter_map(|x| if !x.0 { Some(x.1) } else { None }),
                    );

                monkeys[i].inspections += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }

        let inspections_list: (usize, usize) = monkeys.into_iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .unwrap();

        (inspections_list.0 * inspections_list.1).to_string()
    }
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Add(x) => value + x,
            Operation::Multiply(x) => value * x,
            Operation::Square => value * value,
        }
    }

    fn parse(input: &str) -> Self {
        let parts = input.rsplitn(3, ' ')
            .collect_vec();
            
        match parts[1] {
            "+" => {
                let value = parts[0].parse::<u64>().unwrap();
                Operation::Add(value)
            },
            "*" => match parts[0] {
                "old" => Operation::Square,
                x => {
                    let value = x.parse::<u64>().unwrap();
                    Operation::Multiply(value)
                }
            }
            _ => panic!("No valid operation")
        }
    }
}

fn parse_monkey(input: &str) -> Monkey {
    let lines = input.lines().collect_vec();
    let items = &lines[1][18..].split(", ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect_vec();

    Monkey {
        items: items.clone(),
        opertation: Operation::parse(&lines[2][19..]),
        divisor: parse_string_to_number(&lines[3][21..]),
        throw_to: (parse_string_to_number(&lines[4][29..]) as usize, parse_string_to_number(&lines[5][30..]) as usize),
        inspections: 0,
    }
}

fn parse_string_to_number(input: &str) -> u64 {
    let regex = Regex::new(r"\d+").unwrap();

    regex.find(input).unwrap()
        .as_str()
        .parse::<u64>()
        .unwrap()
}