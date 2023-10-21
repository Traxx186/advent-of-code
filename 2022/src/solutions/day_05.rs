use aoc_common::{
    collections::{pop_multiple, push_multiple},
    load, 
    Solution
};
use itertools::Itertools;
use regex::Regex;

pub struct Day05 {}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Solution for Day05 {

    fn name(&self) -> String {
        "Supply Stacks".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_05");
        let crates_and_moves: (&str, &str) = input.split("\n\n").collect_tuple().unwrap();
        let mut stack = get_crate_stack(crates_and_moves.0.to_string());
        let move_orders = parse_moves(&crates_and_moves.1.lines().collect_vec());

        for move_order in move_orders {
            let stack_to_move = pop_multiple(&mut stack[move_order.from - 1], move_order.amount).unwrap();
            push_multiple(&mut stack[move_order.to - 1], stack_to_move);
        }

        let result = stack.into_iter()
            .map(|item| item.last().unwrap().clone())
            .collect_vec();

        String::from_iter(result)
    }

    fn part_2(&self) -> String {
        let input = load("day_05");
        let crates_and_moves: (&str, &str) = input.split("\n\n").collect_tuple().unwrap();
        let mut stack = get_crate_stack(crates_and_moves.0.to_string());
        let move_orders = parse_moves(&crates_and_moves.1.lines().collect_vec());

        for move_order in move_orders {
            let stack_to_move = pop_multiple(&mut stack[move_order.from - 1], move_order.amount).unwrap();
            use_crane_9001(&mut stack[move_order.to - 1], stack_to_move);
        }

        let result = stack.into_iter()
            .map(|item| item.last().unwrap().clone())
            .collect_vec();

        String::from_iter(result)
    }
}

fn get_crate_stack(input: String) -> Vec<Vec<char>> {
    let mut data = input.lines()
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .map(|chars| chars.chunks(4).map(|chunky_boy| chunky_boy[1]).collect_vec())
        .collect_vec();

    let mut illegal_immigrants = vec![vec![]; data.last().unwrap().len()];
    data.pop();

    for item in data.iter() {
        for (i, el) in item.iter().enumerate() {
            if !el.is_whitespace() {
                illegal_immigrants[i].push(el.to_owned())
            }
        }
    }

    let sorted_stack = illegal_immigrants
        .into_iter()
        .map(|stack| stack.iter().copied().rev().to_owned().collect_vec())
        .collect_vec();

    sorted_stack
}

fn parse_moves(input: &Vec<&str>) -> Vec<Move> {
    let mut moves = Vec::new();
    for item in input {
        let regex = Regex::new(r"\d+").unwrap();
        let move_action = regex.find_iter(item)
            .map(|el| el.as_str().parse::<usize>().unwrap())
            .collect_vec();

        moves.push(Move { amount: move_action[0], from: move_action[1], to: move_action[2] })
    }

    moves
}

fn use_crane_9001(out: &mut Vec<char>, mut items:  Vec<char>) {
    items.reverse();
    push_multiple(out, items);
}