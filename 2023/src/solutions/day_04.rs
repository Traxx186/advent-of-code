use std::iter;

use aoc_common::{Solution, load};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: isize,
    pub winning_numbers: Vec<isize>,
    pub scratch_numbers: Vec<isize>,
}

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> String {
        "Scratchcards".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_04");
        let games = generate_games(input);
        let mut sum = 0;

        for game in games {
            let matching = game.scratch_numbers
                .iter()
                .filter(|num| game.winning_numbers.contains(&num))
                .count() as u32;

            sum += if matching > 0 {
                2_i32.pow(matching.saturating_sub(1))
            } else {
                0
            };
        }
        
        sum.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_04");
        let games = generate_games(input);
        let mut count = iter::repeat(1).take(games.len()).collect_vec();

        for i in 0..games.len() {
            let game = &games[i];
            let matching = game.scratch_numbers
                .iter()
                .filter(|num| game.winning_numbers.contains(&num))
                .count();

            let add = count[i];
            for j in (1 + i)..=(i + matching) {
                count.get_mut(j).map(|c| *c += add);
            }
        }

        let sum: isize = count.iter().sum();
        sum.to_string()
    }
}

fn generate_games(input: String) -> Vec<Game> {
    let mut games = vec![];

    for line in input.lines() {
        let (game_str, numbers_str) = line.split_once(": ").unwrap();
        let (winning_str, sratch_str) = numbers_str.split_once(" | ").unwrap();

        let id: isize = game_str.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        let winning_numbers = winning_str.split(' ')
            .filter(|split| !split.is_empty())
            .map(|split| split.parse::<isize>().unwrap())
            .collect_vec();

        let scratch_numbers = sratch_str.split(' ')
            .filter(|split| !split.is_empty())
            .map(|split| split.parse::<isize>().unwrap())
            .collect_vec();

        games.push(Game { id, winning_numbers, scratch_numbers })
    }

    games
}