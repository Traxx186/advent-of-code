use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day03;

#[derive(Debug, PartialEq)]
struct PartNumber {
    start: usize,
    end: usize,
    value: isize,
}

impl Solution for Day03 {
    fn name(&self) -> String {
        "Gear Ratios".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_03");
        let mut part_numbers: Vec<isize> = vec![];

        for (i, line) in input.lines().enumerate() {
            let parts = get_parts_from_line(line);

            for part in parts {
                let left = part.start.saturating_sub(1);
                let right = if part.end + 1 >= line.len() {
                    line.len() - 1
                } else {
                    part.end + 1
                };

                let has_symbol_left = line.chars()
                    .nth(left)
                    .is_some_and(is_symbol);

                let has_symbol_right = line.chars()
                    .nth(right)
                    .is_some_and(is_symbol);

                let has_symbol_top = input
                    .lines()
                    .collect_vec()
                    .get(i.saturating_sub(1))
                    .is_some_and(|&l| {
                        l.get(left..=right)
                            .is_some_and(|v| v.chars().any(is_symbol))
                    });

                let has_symbol_bottom = input.lines().collect_vec().get(i + 1).is_some_and(|&l| {
                    l.get(left..=right)
                        .is_some_and(|v: &str| v.chars().any(is_symbol))
                });

                if has_symbol_left || has_symbol_right || has_symbol_top || has_symbol_bottom {
                    part_numbers.push(part.value);
                }
            }
        }

        let sum: isize = part_numbers.iter().sum();
        sum.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_03");
        let lines = input.lines().collect_vec();
        let parts = lines.iter()
            .map(|l| get_parts_from_line(l))
            .collect_vec();
        
        let mut sum = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate()  {
                if !is_symbol(char) {
                    continue;
                }

                let mut found_parts = vec![];
                let left = x.saturating_sub(1);
                let right = if x + 1 >= line.len() { line.len() - 1 } else { x + 1 };
                let top = y.saturating_sub(1);
                let bottom = if y + 1 >= lines.len() { lines.len() - 1 } else { y + 1};

                found_parts.append(&mut vec![
                    parts[y].iter().find(|p| p.end == left),
                    parts[y].iter().find(|p| p.start == right),
                    parts[top].iter().find(|p| (p.start..=p.end).contains(&x)),
                    parts[top].iter().find(|p| p.end == left),
                    parts[top].iter().find(|p| p.start == right),
                    parts[bottom].iter().find(|p| (p.start..=p.end).contains(&x)),
                    parts[bottom].iter().find(|p| p.end == left),
                    parts[bottom].iter().find(|p| p.start == right)
                ]);

                let adjacent_parts = found_parts.iter()
                    .filter_map(|part| part.map(|item| item))
                    .collect_vec();

                if adjacent_parts.len() >= 2 {
                    sum += adjacent_parts.iter()
                        .map(|p| p.value)
                        .product::<isize>()
                }
            }
        }

        sum.to_string()
    }
}

fn get_parts_from_line(line: &str) -> Vec<PartNumber> {
    let mut result = vec![];
    let mut number_string = String::new();
    let mut start = 0;
    let mut end = 0;

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            if number_string.is_empty() {
                start = i;
            }

            end = i;
            number_string.push(char);
        } else if !number_string.is_empty() {
            let value: isize = number_string.parse::<isize>().unwrap();

            result.push(PartNumber { start, end, value });
            number_string = String::new();
            start = 0;
            end = 0;
        }
    }

    if !number_string.is_empty() {
        let value: isize = number_string.parse::<isize>().unwrap();

        result.push(PartNumber { start, end, value });
    }

    result
}

fn is_symbol(item: char) -> bool {
    !item.is_ascii_digit() && item != '.'
}
