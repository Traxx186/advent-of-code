use std::collections::HashSet;

use aoc_common::{
    direction::Direction,
    Solution,
    load
};
use itertools::Itertools;

pub struct Day09 {}

#[derive(Debug, Clone, Copy)]
struct Movement {
    steps: usize,
    direction: Direction
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Solution for Day09 {

    fn name(&self) -> String {
        "Rope Bridge".to_owned()
    }

    fn part_1(&self) -> String {
        let movements = parse_input_to_moves(load("day_09"));
        
        let mut head = Point::new();
        let mut tail = Point::new();

        let mut visited_points: HashSet<Point> = HashSet::new();
        visited_points.insert(tail.clone()); 

        for movement in movements {
            for _ in 0..movement.steps {
                head = head.move_direction(movement.direction);
                tail = move_knot(head, tail.clone());
    
                visited_points.insert(tail);   
            }
        }

        visited_points.len().to_string()
    }

    fn part_2(&self) -> String {
        let movements = parse_input_to_moves(load("day_09"));

        let mut head = Point::new();
        let mut knots = vec![Point::new(); 9];

        let mut visited_points: HashSet<Point> = HashSet::new();
        visited_points.insert(knots[knots.len() - 1]); 

        for movement in movements {
            for _ in 0..movement.steps {
                head = head.move_direction(movement.direction);

                for i in 0..knots.len() {
                    let leader = if i == 0 { head } else { knots[i - 1] };
                    knots[i] = move_knot(leader, knots[i]);
                }

                visited_points.insert(knots[knots.len() - 1]);
            }
        }

        visited_points.len().to_string()
    }
}

fn parse_input_to_moves(input: String) -> Vec<Movement> {
    return input.lines()
        .map(|line| line.split(' ').collect_tuple::<(&str, &str)>().unwrap()).collect_vec()
        .iter()
        .map(|move_item| Movement { direction: get_move_direction(move_item.0), steps: move_item.1.parse().unwrap()})
        .collect_vec();
}

fn get_move_direction(direction: &str) -> Direction {
    match direction {
        "U" => Direction::North,
        "D" => Direction::South,
        "L" => Direction::West,
        "R" => Direction::East,
        _ => panic!("No valid move direction")
    }
}

fn move_knot(lead: Point, follower: Point) -> Point {
    let delta_x = lead.x - follower.x;
    let delta_y = lead.y - follower.y;

    if delta_x.abs() == 2 || delta_y.abs() == 2 {   
        return follower.move_to(delta_x.signum(), delta_y.signum());
    }

    follower
}

impl Point {
    
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn move_to(&self, delta_x: isize, delta_y: isize) -> Point {
        Point { x: self.x + delta_x, y: self.y + delta_y}
    }

    fn move_direction(&self, direction: Direction) -> Point {
        match direction {
            Direction::North => self.move_to(0, 1),
            Direction::South => self.move_to(0, -1),
            Direction::West => self.move_to(-1, 0),
            Direction::East => self.move_to(1, 0),
            _ => panic!("Invalid directon")
        }
    }
}