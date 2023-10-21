use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day20 {}

impl Solution for Day20 {

    fn name(&self) -> String {
        "Grove Positioning System".to_owned()
    }

    fn part_1(&self) -> String {
        let numbers = parse_input(load("day_20"), 1);
        let result = decrypt(numbers, 1);

        result.to_string()
    }

    fn part_2(&self) -> String {
        let numbers = parse_input(load("day_20"), 811_589_153);
        let result = decrypt(numbers, 10);

        result.to_string()
    }
}

#[derive(Debug)]
struct NumberInput {
    value: isize,
    original_position: usize
}

fn parse_input(input: String, scale: isize) -> Vec<NumberInput> {
    input.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .enumerate()
        .map(|(i, value)| NumberInput { value: value * scale, original_position: i })
        .collect_vec()
}

fn decrypt(mut numbers: Vec<NumberInput>, cycles: isize) -> isize {
    let message_size = numbers.len() as isize - 1;
    for _ in 0..cycles {
        for current in 0..numbers.len() {
            let index = numbers.iter().position(|x| x.original_position == current).unwrap();
            let mut new_index = index as isize + numbers[index].value;
            new_index = ((new_index % message_size) + message_size) % message_size;
            let number = numbers.remove(index);
            numbers.insert(new_index as usize, number);
        }
    }

    let zero_index = numbers.iter().position(|x| x.value == 0).unwrap();
    let n1 = numbers[(zero_index + 1000) % numbers.len()].value;
    let n2= numbers[(zero_index + 2000) % numbers.len()].value;
    let n3 = numbers[(zero_index + 3000) % numbers.len()].value;

    n1 + n2 + n3
}