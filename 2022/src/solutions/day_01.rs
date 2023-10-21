use aoc_common::{load, Solution};

pub struct Day01 {}

impl Solution for Day01 {

    fn name(&self) -> String {
        "Calorie Counting".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_01");
        let mut total_cal_per_elf = get_total_cal_per_elf(input);

        total_cal_per_elf.sort_by(|a, b| b.cmp(a));
        total_cal_per_elf.first().unwrap_or(&0 ).to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_01");

        let mut total_cal_per_elf = get_total_cal_per_elf(input);
        total_cal_per_elf.sort_by(|a, b| b.cmp(a));

       let top_three_total_cal: isize = total_cal_per_elf.split_at(3).0.into_iter().sum();
        top_three_total_cal.to_string()
    }
}

fn get_total_cal_per_elf(input: String) -> Vec<isize> {
    let mut cal_per_elf: isize = 0;
    let mut elf_array = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            let value = line.parse::<isize>().unwrap_or_else(|_| panic!("{} is not a number", line));
            cal_per_elf += value;
        } else {
            elf_array.push(cal_per_elf);
            cal_per_elf = 0;
        }
    }

    elf_array
}