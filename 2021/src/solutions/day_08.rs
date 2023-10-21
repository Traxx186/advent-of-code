use std::collections::HashMap;
use aoc_common::{load, Solution};

const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"
];

pub struct Day08 {}

impl Solution for Day08 {

    fn name(&self) -> String {
        "Seven Segment Search".to_owned()
    }

    fn part_1(&self) -> String {
        let input = parse_input(load("day_08"));

        let mut sum: usize = 0;
        for lines in &input {
            sum += lines.1.iter()
                .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                .count();
        }

        sum.to_string()
    }

    fn part_2(&self) -> String {
        let input = parse_input(load("day_08"));
        let mut sum: u32 = 0;

        let perms = permutations(CHARS.to_vec());
        let mut sorted_digits = DIGITS.to_vec();
        sorted_digits.sort();

        for lines in &input {
            for perm in &perms {
                let mut wires = HashMap::new();

                for i in 0..CHARS.len() {
                    *wires.entry(CHARS[i]).or_insert(perm[i]) = perm[i];
                }

                let mut new_clues = Vec::new();
                for clue in &lines.0 {
                    let mut x = String::new();
                    for char in clue.chars() {
                        x.push(*wires.get(&char).unwrap());
                    }

                    let mut to_sort = x.chars().collect::<Vec<char>>();
                    to_sort.sort_unstable();
                    new_clues.push(
                        to_sort.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    );
                }
                new_clues.sort();

                if new_clues == sorted_digits {
                    let mut n = Vec::new();
                    for line in &lines.1 {
                        let mut x = String::new();
                        for char in line.chars() {
                            x.push(*wires.get(&char).unwrap());
                        }
                        let mut to_sort = x.chars().collect::<Vec<char>>();
                        to_sort.sort_unstable();
                        x = to_sort.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join("");

                        n.push(DIGITS.iter().position(|i| **i == x).unwrap());
                    }

                    sum += n.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .parse::<u32>()
                        .unwrap();
                    break;
                }
            }
        }

        sum.to_string()
    }
}

fn parse_input(input: String) -> Vec<(Vec<String>, Vec<String>)> {
    let mut out = Vec::new();

    for line in input.lines() {
        let mut parts = line.split('|');

        let test = parts.next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|y| y.to_string())
            .collect::<Vec<String>>();

        let check = parts.next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|y| y.to_string())
            .collect::<Vec<String>>();

        out.push((test, check));
    }

    out
}

fn permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        return vec![items]
    }

    let mut output: Vec<Vec<T>> = Vec::new();
    let mut unique_items = items.clone();

    unique_items.sort();
    unique_items.dedup();

    for item in unique_items {
        let mut remaining_elements = items.clone();

        let index = remaining_elements.iter().position(|x| *x == item).unwrap();
        remaining_elements.remove(index);

        for mut permutation in permutations(remaining_elements) {
            permutation.insert(0, item.clone());
            output.push(permutation);
        }
    }

    output
}