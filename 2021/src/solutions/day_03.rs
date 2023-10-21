use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day03 {}

impl Solution for Day03 {

    fn name(&self) -> String {
        "Binary Diagnostic".to_owned()
    }

    fn part_1(&self) -> String {
        let report = load("day_03");
        let line_length = report.lines().next().unwrap().len();

        let mut gamma = vec![0; line_length];
        let mut epsilon = vec![1; line_length];

        for i in 0..line_length {
            let mut high = 0;
            let mut low = 0;

            report.lines().for_each(|column| match column.chars().nth(i).unwrap() {
                '0' => low += 1,
                '1' => high += 1,
                _ => {}
            });

            if high > low {
                gamma[i] = 1;
                epsilon[i] = 0;
            }
        }

        let gamma = parse_bits_to_int(&gamma).unwrap();
        let epsilon = parse_bits_to_int(&epsilon).unwrap();

        (gamma * epsilon).to_string()
    }

    fn part_2(&self) -> String {
        let report = load("day_03");
        let lines = report.lines().collect_vec();

        let o2_rating = find_rating_by_bit_criteria(|a, b| if a.len() >= b.len() { a } else { b }, &lines);
        let co2_rating = find_rating_by_bit_criteria(|a, b| if a.len() >= b.len() { b } else { a }, &lines);

        (str_to_int(o2_rating) * str_to_int(co2_rating)).to_string()
    }
}

fn parse_bits_to_int(bits: &[u32]) -> Option<usize> {
    usize::from_str_radix(
        &bits.iter()
                .map(|bit| bit.to_string())
                .collect::<Vec<String>>()
                .join(""),
        2,
    ).ok()
}

fn find_rating_by_bit_criteria<'a>(
    criteria: impl Fn(Vec<&'a str>, Vec<&'a str>) -> Vec<&'a str>,
    candidates: &[&'a str]
) -> &'a str {
    let mut i = 0;
    let mut survivors = candidates.to_vec();

    while survivors.len() > 1 {
        let (one_dominant, zero_dominant): (Vec<&str>, Vec<&str>) = survivors
            .iter()
            .partition(|s| s.chars().nth(i).unwrap() == '1');

        survivors = criteria(one_dominant, zero_dominant);
        i += 1;
    }

    survivors.pop().unwrap()
}

fn str_to_int(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}