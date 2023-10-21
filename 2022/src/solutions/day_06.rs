use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day06 {}

impl Solution for Day06 {

    fn name(&self) -> String {
        "Tuning Trouble".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_06");
        let chars = input.chars().collect_vec();
        let unique_char_pos = get_package_from_string(chars, 4);

        unique_char_pos.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_06");
        let chars = input.chars().collect_vec();
        let unique_char_pos = get_package_from_string(chars, 14);

        unique_char_pos.to_string()
    }

}

fn get_package_from_string(chars: Vec<char>, package_size: usize) -> usize {
    let mut unique_char_pos = 0;

    for i in 0..chars.len() {
        let end = if (i + 4) > chars.len() {
            chars.len() - 1
        } else {
            i + package_size
        };

        let pair = chars[i..end].into_iter().collect_vec();
        let unique_pair = pair.clone().into_iter()
            .all_unique();

        if unique_pair {
            unique_char_pos = i + pair.len();
            break;
        }
    }

    unique_char_pos
}