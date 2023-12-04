use std::collections::HashMap;

use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day02;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
}

struct Game {
    pub id: isize,
    pub reveals: Vec<HashMap<Color, isize>>
}

impl Solution for Day02 {
    fn name(&self) -> String {
        "Cube Conundrum".to_owned()
    }

    fn part_1(&self) -> String {
        let bag = HashMap::from([
            (Color::Blue, 14),
            (Color::Red, 12),
            (Color::Green, 13)
        ]);

        let input = load("day_02");
        let game = input.lines()
            .map(|line| parse_game(line))
            .collect_vec();

        let possible_games: isize = game.iter()
            .filter_map(|g| is_game_possible(g, &bag))
            .sum();

        possible_games.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_02");
        let games = input.lines()
            .map(|line| parse_game(line))
            .collect_vec();

        let total_power: isize = games.iter()
            .map(|game| get_smallest_bag(game))
            .map(|bag| bag.values().product::<isize>())
            .sum();

        total_power.to_string()
    }
}

fn parse_game(line: &str) -> Game {
    let (game_part, reveals_part) = line.split_once(':').unwrap();
    let id: isize = game_part.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();
    
    let reveals = reveals_part.split(';')
        .map(|r| r.split(',').map(parse_color_reveal).collect())
        .collect();

    Game { id, reveals }
}

fn parse_color_reveal(reveal: &str) -> (Color, isize) {
    let (amount_str, color_str) = reveal.trim().split_once(' ').unwrap();

    (color_str.into(), amount_str.parse().unwrap())
}

fn is_game_possible(game: &Game, bag: &HashMap<Color, isize>) -> Option<isize> {
    let game_possible = game.reveals.iter()
        .flat_map(|reveal| reveal.iter())
        .all(|(color, number)| number <= bag.get(color).unwrap_or(&0));

    if game_possible {
        return Some(game.id);
    }

    None
}

fn get_smallest_bag(game: &Game) -> HashMap<Color, isize> {
    let mut bag = HashMap::new();
    let all_reveals = game.reveals.iter().flat_map(|reveal| reveal.iter());

    for (color, number) in all_reveals {
        let max = bag.entry(*color).or_insert(*number);
        if *max < *number {
            *max = *number;
        }
    }

    bag
}

impl Into<Color> for &str {
    fn into(self) -> Color {
        match self {
            "blue" => Color::Blue,
            "green" => Color::Green,
            "red" => Color::Red,
            _ => panic!("unknown color {}", self),
        }
    }
}