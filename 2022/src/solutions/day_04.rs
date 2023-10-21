use std::cmp::{max, min};
use std::ops::Range;

use aoc_common::{load, Solution};

pub struct Day04 {}

impl Solution for Day04 {
    fn name(&self) -> String {
        "Camp Cleanup".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_04");
        let num_of_overlapping_areas: isize = input
            .lines()
            .filter_map(|row| {
                match row.split_once(',') {
                    Some((range_1, range_2)) => {
                        let range_1 = get_working_section_range(range_1);
                        let range_2 = get_working_section_range(range_2);
                        has_section_overlap(&range_1, &range_2)
                            .map(|int| (int == range_1) | (int == range_2))
                    }
                    _ => panic!("invalid input"),
                }
                .map(|result| result as isize)
            })
            .fold(0, |a, b| a + b);

        num_of_overlapping_areas.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_04");
        let num_of_overlapping_areas = input
            .lines()
            .filter_map(|row| {
                match row.split_once(',') {
                    Some((range_1, range_2)) => {
                        let range_1 = get_working_section_range(range_1);
                        let range_2 = get_working_section_range(range_2);
                        has_section_overlap(&range_1, &range_2)
                    }
                    _ => panic!("invalid input"),
                }
                .map(|_| 1u64)
            })
            .sum::<u64>();

        num_of_overlapping_areas.to_string()
    }
}

fn get_working_section_range(section: &str) -> Range<isize> {
    match section.split_once('-') {
        Some((start, end)) => Range {
            start: start.parse::<isize>().unwrap(),
            end: end.parse::<isize>().unwrap(),
        },
        _ => panic!("Input should contain '-'"),
    }
}

fn has_section_overlap(range_1: &Range<isize>, range_2: &Range<isize>) -> Option<Range<isize>> {
    match (range_2.start > range_1.end) | (range_1.start > range_2.end) {
        true => None,
        false => Some(max(range_1.start, range_2.start)..min(range_1.end, range_2.end)),
    }
}
