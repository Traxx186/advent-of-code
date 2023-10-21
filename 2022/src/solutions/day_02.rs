use aoc_common::{load, Solution};
use itertools::Itertools;

pub struct Day02 {}

impl Solution for Day02 {

    fn name(&self) -> String {
        "Rock Paper Scissors".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_02");
        let mut score_per_round = Vec::new();

        for line in input.lines() {
            let round: (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let mut round_score: isize = 0;

            //A & X: Rock,
            //B & Y: Paper
            //C & Z: Scissors
            match round.1 {
                "X" => {
                    round_score += 1;
                    round_score += match round.0 {
                        "A" => 3,
                        "B" => 0,
                        "C" => 6,
                        _ => 0
                    }
                },
                "Y" => {
                    round_score += 2;
                    round_score += match round.0 {
                        "A" => 6,
                        "B" => 3,
                        "C" => 0,
                        _ => 0,
                    }
                },
                "Z" => {
                    round_score += 3;
                    round_score += match round.0 {
                        "A" => 0,
                        "B" => 6,
                        "C" => 3,
                        _ => 0,
                    }
                },
                _ => panic!("No valid input")
            }

            score_per_round.push(round_score);
        }

        let total_score: isize = score_per_round.iter().sum();
        total_score.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_02");
        let mut score_per_round = Vec::new();

        for line in input.lines() {
            let round: (&str, &str) = line.split(" ").collect_tuple().unwrap();
            let mut round_score: isize = 0;

            //A: Rock (1),
            //B: Paper (2)
            //C: Scissors (3)
            match round.1 {
                "X" => {
                    round_score += match round.0 {
                        "A" => 3,
                        "B" => 1,
                        "C" => 2,
                        _ => 0,
                    }
                }
                "Y" => {
                    round_score += 3;
                    round_score += match round.0 {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => 0,
                    }
                }
                "Z" => {
                    round_score += 6;
                    round_score += match round.0 {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => 0,
                    }
                }
                _ => panic!("No valid input")
            }

            score_per_round.push(round_score);
        }

        let total_score: isize = score_per_round.iter().sum();
        total_score.to_string()
    }
}