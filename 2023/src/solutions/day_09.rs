use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> String {
        "Mirage Maintenance".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_09");
        let mut history: Vec<isize> = vec![];
        
        for line in input.lines() {
            let measurements = get_measurements(line);
            let mut differences: Vec<Vec<isize>> = get_differences(measurements);
        
            for (i, difference) in differences.clone().iter().enumerate() {
                let end_measure = difference.last()
                    .and_then(|previous_measure| {
                        if i == 0 { 
                            Some(&0)
                        } else {
                            differences.get(i - 1)
                                .and_then(|previous_diff| previous_diff.last())
                        }
                        .map(|new_diff| previous_measure + new_diff)
                    })
                    .unwrap();

                differences[i].push(end_measure);
            }

            history.push(
                *differences.last()
                    .and_then(|measurements| measurements.last())
                    .unwrap()
            );
        }

        history.iter()
            .sum::<isize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_09");
        let mut history: Vec<isize> = vec![];
        
            for line in input.lines() {
                let measurements = get_measurements(line);
                let mut differences: Vec<Vec<isize>> = get_differences(measurements);
            
                for (i, difference) in differences.clone().iter().enumerate() {
                    let end_measure = difference.first()
                        .and_then(|previous_measure| {
                            if i == 0 { 
                                Some(&0)
                            } else {
                                differences.get(i - 1)
                                    .and_then(|previous_diff| previous_diff.first())
                            }
                            .map(|new_diff| previous_measure - new_diff)
                        })
                        .unwrap();
    
                    differences[i].insert(0, end_measure);
                }
    
                history.push(
                    *differences.last()
                        .and_then(|measurements| measurements.first())
                        .unwrap()
                );
            }

        history.iter()
            .sum::<isize>()
            .to_string()
    }
}

fn get_measurements(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<isize>().ok())
        .collect_vec()
}

fn get_differences(measurements: Vec<isize>) -> Vec<Vec<isize>> {
    let mut differences = vec![measurements];

    while differences.first().unwrap().iter().sum::<isize>() != 0 {
        let diffs_to_insert = differences.first()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(m1, m2)| m2 - m1)
            .collect_vec();

        differences.insert(0, diffs_to_insert)
    }

    differences
}