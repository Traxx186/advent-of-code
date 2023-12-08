use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> String {
        "Wait For It".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_06");
        let races: (Vec<String>, Vec<String>) = parse_races(&input);
        let mut win_counts: Vec<isize> = vec![];

        for (i, time) in races.0.iter().enumerate() {
            let distance_record = races.1[i].parse::<isize>().unwrap();
            let mut win_count: isize = 0;

            let race_time = time.parse::<isize>().unwrap();
            for hold_time in (0..=race_time).rev() {
                let distance = (race_time - hold_time) * hold_time;
                if distance > distance_record {
                    win_count += 1;
                }
            }

            win_counts.push(win_count);
        }

        win_counts.iter()
            .product::<isize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_06");
        let races = parse_races(&input);
        let mut win_count: isize = 0;

        let time = races.0.join("")
            .parse::<isize>()
            .unwrap();

        let distance_record = races.1.join("")
            .parse::<isize>()
            .unwrap();

        for hold_time in (0..=time).rev() {
            let distance = (time - hold_time) * hold_time;
            if distance > distance_record {
                win_count += 1;
            }
        }

        win_count.to_string()
    }
}

fn parse_races(input: &String) -> (Vec<String>, Vec<String>) {
    let (time_line, distance_line) = input.split('\n')
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let times = time_line.split_once(':')
        .unwrap().1
        .split_whitespace()
        .map(|t| t.to_string())
        .collect_vec();

    let distances = distance_line.split_once(':')
        .unwrap().1
        .split_whitespace()
        .map(|d| d.to_string())
        .collect_vec();

    (times, distances)
}