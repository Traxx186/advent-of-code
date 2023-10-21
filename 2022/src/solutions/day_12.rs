use std::collections::{HashMap, VecDeque};

use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day12 {}
type Position = (isize, isize);

impl Solution for Day12 {
    
    fn name(&self) -> String {
        "Hill Climbing Algorithm".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_12");
        let readings = input.lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut grid: HashMap<Position, char> = HashMap::new();
        let mut start: Position = (0, 0);
        let mut end: Position = (0, 0);

        for (y, reading) in readings.iter().enumerate() {
            for (x, item) in reading.iter().enumerate() {
                let mut height = item;

                if item == &'S' {
                    start = (x as isize, y as isize);
                    height = &'a';
                } else if item == &'E' {
                    end = (x as isize, y as isize);
                    height = &'z'
                }

                grid.insert((x as isize, y as isize), *height);
            }
        }

        find_path(&grid, start, end).unwrap().to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_12");
        let readings = input.lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut grid: HashMap<Position, char> = HashMap::new();
        let mut possible_start: Vec<Position> = Vec::new();
        let mut end: Position = (0, 0);

        for (y, reading) in readings.iter().enumerate() {
            for (x, item) in reading.iter().enumerate() {
                let mut height = item;

                if item == &'S' {
                    height = &'a';
                } else if item == &'E' {
                    end = (x as isize, y as isize);
                    height = &'z'
                }

                if height == &'a' {
                    possible_start.push((x as isize, y as isize));
                }

                grid.insert((x as isize, y as isize), *height);
            }
        }

        possible_start.into_iter()
            .map(|start| find_path(&grid, start, end))
            .filter(|distance| distance.is_some())
            .min()
            .unwrap()
            .unwrap()
            .to_string()
    }
}

fn find_path(grid: &HashMap<Position, char>, start: Position, end: Position) -> Option<usize> {
    let mut visited: HashMap<Position, usize> = HashMap::new();
    let mut to_visit: VecDeque<Position> = VecDeque::new();
    
    visited.insert(start, 0);
    to_visit.push_back(start);

    while !to_visit.is_empty() {
        let position = to_visit.pop_front().unwrap();

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_position = (position.0 + dx, position.1 + dy);
            if grid.contains_key(&new_position) && !visited.contains_key(&new_position) {
                if *grid.get(&new_position).unwrap() as isize - *grid.get(&position).unwrap() as isize <= 1 {
                    to_visit.push_back(new_position);
                    visited.insert(new_position, visited.get(&position).unwrap() + 1);

                    if new_position == end {
                        return Some(visited.get(&position).unwrap() + 1);
                    }
                }
            }
        }
    }

    None
}