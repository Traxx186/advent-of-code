use std::collections::HashSet;

use aoc_common::{
    point::Point2D,
    Solution, load
};
use nom::IResult;
use nom::bytes::complete::tag;
use nom::sequence::{tuple, preceded};

pub struct Day14 {}

impl Solution for Day14 {

    fn name(&self) -> String {
        "Regolith Reservoir".to_owned()
    }

    fn part_1(&self) -> String {
        let mut world: HashSet<Point2D<i32>, _> = HashSet::new();

        generate_world(&mut world, load("day_14"));

        let lowest = find_lowest_point(world.clone());
        let mut sand = Point2D { x: 500, y: 0 };
        let mut sand_history = vec![sand]; 
        let mut total_grians_of_sand = 0;

        while sand.y < lowest {
            let point_down = Point2D { x: sand.x, y: sand.y + 1 };
            let point_left = Point2D { x: sand.x - 1, y: sand.y + 1 };
            let point_right = Point2D { x: sand.x + 1, y: sand.y + 1 };

            if !world.contains(&point_down) {
                sand_history.push(sand);
                sand = point_down;
            } else if !world.contains(&point_left) {
                sand_history.push(sand);
                sand = point_left;
            } else if !world.contains(&point_right) {
                sand_history.push(sand);
                sand = point_right;
            } else {
                world.insert(sand);
                total_grians_of_sand += 1;
                if let Some(prev) = sand_history.pop() {
                    sand = prev;
                } else {
                    sand = Point2D { x: 500, y: 0};
                }
            }
        }

        total_grians_of_sand.to_string()
    }

    fn part_2(&self) -> String {
        let mut world: HashSet<Point2D<i32>, _> = HashSet::new();

        generate_world(&mut world, load("day_14"));

        let lowest = find_lowest_point(world.clone());
        let mut sand = Point2D { x: 500, y: 0 };
        let mut sand_history = vec![sand]; 

        let mut total_grians_of_sand = 0;
        loop {
            let point_down = Point2D { x: sand.x, y: sand.y + 1 };
            let point_left = Point2D { x: sand.x - 1, y: sand.y + 1 };
            let point_right = Point2D { x: sand.x + 1, y: sand.y + 1 };

            if sand.y == lowest + 1 {
                if let Some(prev) = sand_history.pop() {
                    world.insert(sand);
                    total_grians_of_sand += 1;
                    sand = prev;
                } else {
                    break;
                }

                continue;
            }
            

            if !world.contains(&point_down) {
                sand_history.push(sand);
                sand = point_down;
            } else if !world.contains(&point_left) {
                sand_history.push(sand);
                sand = point_left;
            } else if !world.contains(&point_right) {
                sand_history.push(sand);
                sand = point_right;
            } else if let Some(prev) = sand_history.pop() {
                world.insert(sand);
                total_grians_of_sand += 1;
                sand = prev;
            } else {
                break;
            }
        }

        total_grians_of_sand.to_string()
    }
}

fn parse_coordinates(input: &str) -> IResult<&str, (i32, i32)> {
    tuple((
        nom::character::complete::i32,
        preceded(tag(","), nom::character::complete::i32)
    ))(input)
}

fn parse_sequence(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(tag(" -> "), parse_coordinates)(input)
}

fn generate_world(world: &mut HashSet<Point2D<i32>>, input: String) {
    for line in input.lines() {
        let (mut next, start) = parse_coordinates(line).unwrap();
        let mut point = start.into();

        world.insert(point);
        while let Ok((_next, pair)) = parse_sequence(next) {
            let next_point: Point2D<_> = pair.into();

            while point != next_point {
                point += next_point.sigsum(&point);
                world.insert(point);
            }

            next = _next;
        }
    }
}

fn find_lowest_point(world: HashSet<Point2D<i32>>) -> i32 {
    world.clone()
        .into_iter()
        .map(|point| point.y)
        .max()
        .unwrap()
}