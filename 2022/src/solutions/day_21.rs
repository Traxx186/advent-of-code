use std::collections::HashMap;

use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day21 {}

impl Solution for Day21 {

    fn name(&self) -> String {
        "Monkey Math".to_owned()
    }

    fn part_1(&self) -> String {
        let monkeys = parse_input(load("day_21"));
        let result = calculate_riddle("root", &monkeys);

        result.to_string()
    }

    fn part_2(&self) -> String {
        let monkeys = parse_input(load("day_21"));
        let mut human_path = find_path("root", &monkeys).unwrap();
        let number_to_yell;

        human_path.pop();

        let (left, right) = match monkeys.get("root") {
            Some(Monkey::Operation((_, name_1, name_2))) => (name_1, name_2),
            _ => panic!("Root monkey not found")
        };

        if left == &human_path.pop().unwrap() {
            let right_result = calculate_riddle(right, &monkeys);
            number_to_yell = calculate_yell_number(left, &monkeys, &mut human_path, right_result);
        } else {
            let left_result = calculate_riddle(left, &monkeys);
            number_to_yell = calculate_yell_number(right, &monkeys, &mut human_path, left_result);
        }

        number_to_yell.to_string()
    }   
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug, Clone)]
enum Monkey {
    Number(isize),
    Operation((Operation, String, String)),
}

fn parse_input(input: String) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let (name, job) = line.split(": ").collect_tuple().unwrap();
        let monkey = match job.split(' ').collect_vec()[..] {
            [name_1, "+", name_2] => Monkey::Operation((Operation::Add, name_1.to_owned(), name_2.to_owned())),
            [name_1, "-", name_2] => Monkey::Operation((Operation::Subtract, name_1.to_owned(), name_2.to_owned())),
            [name_1, "*", name_2] => Monkey::Operation((Operation::Multiply, name_1.to_owned(), name_2.to_owned())),
            [name_1, "/", name_2] => Monkey::Operation((Operation::Divide, name_1.to_owned(), name_2.to_owned())),
            [number] => Monkey::Number(number.parse().unwrap()),
            _ => panic!("No valid input")
        };

        monkeys.insert(name.to_string(), monkey);
    }

    monkeys
}

fn calculate_riddle(name: &str, monkeys: &HashMap<String, Monkey>) -> isize {
    let monkey = monkeys.get(name).unwrap();
    
    match monkey {
        Monkey::Number(value) => *value,
        Monkey::Operation((Operation::Add, name_1, name_2)) => calculate_riddle(name_1, monkeys) + calculate_riddle(name_2, monkeys),
        Monkey::Operation((Operation::Subtract, name_1, name_2)) => calculate_riddle(name_1, monkeys) - calculate_riddle(name_2, monkeys),
        Monkey::Operation((Operation::Multiply, name_1, name_2)) => calculate_riddle(name_1, monkeys) * calculate_riddle(name_2, monkeys),
        Monkey::Operation((Operation::Divide, name_1, name_2)) => calculate_riddle(name_1, monkeys) / calculate_riddle(name_2, monkeys),
    }
}

fn find_path(name: &str, monkeys: &HashMap<String, Monkey>) -> Option<Vec<String>> {
    if name == "humn" {
        return Some(vec![name.to_owned()]);
    };

    match monkeys.get(name).unwrap() {
        Monkey::Number(_) => None,
        Monkey::Operation((_, name_1, name_2)) => {
            if let Some(mut path) = find_path(name_1, monkeys) {
                path.push(name.to_owned());
                return Some(path);
            } else if let Some(mut path) = find_path(name_2, monkeys) {
                path.push(name.to_owned());
                return Some(path);
            } else {
                return None;
            }
        }
    }
}

fn calculate_yell_number(
    name: &str, 
    monkeys: &HashMap<String, Monkey>, 
    human_path: &mut Vec<String>, 
    result: isize
) -> isize {
    if name == "humn" {
        return result;
    }

    let (operation, left, right) = match monkeys.get(name).unwrap() {
        Monkey::Operation((operation, name_1, name_2)) => (operation, name_1, name_2),
        Monkey::Number(_) => unreachable!()
    };

    let human_left_path = left == &human_path.pop().unwrap();
    let (path_to_solve, other_result) = if human_left_path {
        (left, calculate_riddle(right, monkeys))
    } else {
        (right, calculate_riddle(left, monkeys))
    };

    match (operation, human_left_path) {
        (Operation::Add, _) => calculate_yell_number(path_to_solve, monkeys, human_path, result - other_result),
        (Operation::Subtract, true) => calculate_yell_number(path_to_solve, monkeys, human_path, result + other_result),
        (Operation::Subtract, false) => calculate_yell_number(path_to_solve, monkeys, human_path, other_result - result),
        (Operation::Multiply, _) => calculate_yell_number(path_to_solve, monkeys, human_path, result / other_result),
        (Operation::Divide, true) => calculate_yell_number(path_to_solve, monkeys, human_path, result * other_result),
        (Operation::Divide, false) => calculate_yell_number(path_to_solve, monkeys, human_path, other_result / result),
    }
}