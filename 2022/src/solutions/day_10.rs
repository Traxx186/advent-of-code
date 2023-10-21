use std::ops::Range;

use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day10 {}

impl Solution for Day10 {
    fn name(&self) -> String {
        "Cathode-Ray Tube".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_10");
        let mut register_x = 1;
        let mut sum_signal_strength = 0;
        let mut cycles = 0;

        for line in input.lines() {
            if line.starts_with("addx") {
                cycles += 1;
                sum_signal_strength += calculate_signal_strength(cycles, register_x);

                cycles += 1;
                sum_signal_strength += calculate_signal_strength(cycles, register_x);

                register_x += get_digit_from_line(line);
            } else {
                cycles += 1;
                sum_signal_strength += calculate_signal_strength(cycles, register_x);
            }
        }

        sum_signal_strength.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_10");

        let mut cycles = 0;
        let mut register_x = 1;
        let mut output = String::new();

        for line in input.lines() {
            if line.starts_with("addx") {
                cycles += 1;
                render_pixel(&register_x, &cycles, &mut output);

                cycles += 1;
                render_pixel(&register_x, &cycles, &mut output);

                register_x += get_digit_from_line(line);
            } else {
                cycles += 1;
                render_pixel(&register_x, &cycles, &mut output);
            }
        }

        output.to_string()
    }
}

fn calculate_signal_strength(cycle: i32, register: i32) -> i32 {
    match cycle {
        20 => 20 * register,
        60 => 60 * register,
        100 => 100 * register,
        140 => 140 * register,
        180 => 180 * register,
        220 => 220 * register,
        _ => 0,
    }
}

fn get_digit_from_line(line: &str) -> i32 {
    let split: (&str, &str) = line.split(' ').collect_tuple().unwrap();

    split.1.parse::<i32>().unwrap()
}

fn render_pixel(registry: &i32, cycle: &i32, output: &mut String) {
    let pixel = (*cycle - 1) % 40;
    let range = Range { start: registry - 1, end: registry + 1 };

    if pixel == 0 {
        *output += "\n";
    }

    *output += match pixel >= range.start && pixel <= range.end {
        true => "#",
        false => "."
    }
}