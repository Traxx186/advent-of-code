use aoc_common::{load, Solution};

const CHUCK_CHARS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

pub struct Day10 {}

impl Solution for Day10 {

    fn name(&self) -> String {
        "Syntax Scoring".to_owned()
    }

    fn part_1(&self) -> String {
        let lines = parse_input(load("day_10"));

        let mut total = 0;
        for line in lines {
            let mut closing_chars = Vec::new();
            for char in line.chars() {
                if is_opening_char(char) {
                    closing_chars.push(char_for_char(char));
                    continue;
                }

                if closing_chars.is_empty() || char != closing_chars.pop().unwrap() {
                    total += match char {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!()
                    };

                    break;
                }
            }
        }

        total.to_string()
    }

    fn part_2(&self) -> String {
        let lines = parse_input(load("day_10"));

        let mut scores = Vec::new();
        for line in lines {
            let mut queue = Vec::new();
            let mut is_corrupted = false;
            for char in line.chars() {
                if is_opening_char(char) {
                    queue.push(char_for_char(char));
                    continue;
                }

                if queue.is_empty() || char != queue.pop().unwrap() {
                    is_corrupted = true;
                    break;
                }
            }

            if !is_corrupted {
                let mut score: usize = 0;
                while !queue.is_empty() {
                    let item = queue.pop().unwrap();
                    score = 5 * score + match item {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!()
                    }
                }

                scores.push(score);
            }
        }

        scores.sort();
        scores[scores.len() / 2].to_string()
    }
}

fn parse_input(input: String) -> Vec<String> {
    input.lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn char_for_char(input: char) -> char {
    for i in CHUCK_CHARS {
        if i.0 == input {
            return i.1;
        }
        if i.1 == input {
            return i.0;
        }
    }

    unreachable!();
}

fn is_opening_char(input: char) -> bool {
    for i in CHUCK_CHARS {
        if i.0 == input {
            return true;
        }
    }

    false
}