use aoc_common::{load, Solution};

pub struct Day11 {}

impl Solution for Day11 {

    fn name(&self) -> String {
        "Dumbo Octopus".to_owned()
    }

    fn part_1(&self) -> String {
        let mut data = parse_input(load("day_11"));
        let size = 10;

        let mut total_flashes = 0;

        for _ in 1..=100 {
            for y in 0..size {
                for x in 0..size {
                    data[y * size + x] += 1;
                }
            }

            total_flashes += count_flashes(&mut data, size);
        }

        total_flashes.to_string()
    }

    fn part_2(&self) -> String {
        let mut data = parse_input(load("day_11"));
        let mut index = 1;
        let size = 10;

        let total_octopuses = (size * size) as u32;

        loop {
            for y in 0..size {
                for x in 0..size {
                    data[y * size + x] += 1
                }
            }

            if count_flashes(&mut data, size) == total_octopuses {
                break;
            }

            index += 1;
        }

        index.to_string()
    }
}

fn parse_input(input: String) -> Vec<u32> {
    input.replace('\n', "")
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

fn count_flashes(input: &mut Vec<u32>, size: usize) -> u32 {
    let mut flashes = 0;
    let max_idx = size - 1;

    for y in 0..size {
        for x in 0..size {
            let idx = y * size + x;

            if input[idx] >= 10 {
                input[idx] = 0;
                flashes += 1;

                if x > 0 && !flashing(input[idx - 1]) { input[idx - 1] += 1; }
                if x < max_idx && !flashing(input[idx + 1]) { input[idx + 1] += 1; }

                if y > 0 && !flashing(input[idx - size]) { input[idx - size] += 1; }
                if y < max_idx && !flashing(input[idx + size]) { input[idx + size] += 1; }

                if x > 0 && y > 0 && !flashing(input[idx - size - 1]) { input[idx - size - 1] += 1; }
                if x > 0 && y < max_idx && !flashing(input[idx + size - 1]) { input[idx + size - 1] += 1; }

                if x < max_idx && y > 0 && !flashing(input[idx - size + 1]) { input[idx - size + 1] += 1; }
                if x < max_idx && y < max_idx && !flashing(input[idx + size + 1]) { input[idx + size + 1] += 1; }

                flashes += count_flashes(input, size);
            }
        }
    }

    flashes
}

fn flashing(num: u32) -> bool {
    num == 0
}