use aoc_common::{
    direction::Direction,
    Solution,
    load,
};
use itertools::Itertools;
use num::integer::div_floor;

pub struct Day17 { }

impl Solution for Day17 {

    fn name(&self) -> String {
        "Pyroclastic Flow".to_owned()
    }

    fn part_1(&self) -> String {
        let directions = parse_input(load("day_17"));
        let bricks = make_bricks();

        let mut chamber: Vec<Vec<bool>> = Vec::new();
        let mut direction_index = 0;

        for turn in 0..2022 {
            chamber.play_brick(
                &bricks[turn % bricks.len()],
                &directions, 
                &mut direction_index
            );
        }

        chamber.tower_height().to_string()
    }

    fn part_2(&self) -> String {
        let directions = parse_input(load("day_17"));
        let bricks = make_bricks();
        let mut chamber: Vec<Vec<bool>> = Vec::new();
    
        let mut turn = 0;
        let mut direction_index = 0;

        // Check for a cycle
        let mut cycle: Option<Vec<Vec<bool>>> = None;
        let mut cycle_repeats_at = 0;
        while cycle.is_none() {
            chamber.play_brick(
                &bricks[turn % bricks.len()],
                &directions,
                &mut direction_index,
            );

            turn += 1;

            cycle = chamber.repeats();
            cycle_repeats_at = turn;
        }

        turn = 0;
        direction_index = 0;
        chamber.clear();

        let pattern = cycle.expect("Expected to have found the cycle already!");
        let mut additional_inferred_height = 0;
        while turn < 1_000_000_000_000 {
            chamber.play_brick(
                &bricks[turn % bricks.len()],
                &directions,
                &mut direction_index,
            );
            
            turn += 1;

            if chamber.tower_height() >= pattern.len()
                && chamber[chamber.tower_height() - pattern.len()..chamber.tower_height()] == pattern
            {
                let cycle_interval = cycle_repeats_at - turn;
                let cycle_height = pattern.len();
                let turns_remaining = 1_000_000_000_000 - turn;
                let can_skip_cycles = div_floor(turns_remaining, cycle_interval);
                additional_inferred_height = can_skip_cycles * cycle_height;
                turn += can_skip_cycles * cycle_interval;
            }
        }    

        (chamber.tower_height() + additional_inferred_height).to_string()
    }
}

trait Chaimber {
    fn repeats(&self) -> Option<Vec<Vec<bool>>>;
    fn play_brick(&mut self, brick: &Vec<Vec<bool>>, directions: &Vec<Direction>, direction_index: &mut usize);
    fn tower_height(&self) -> usize;
    fn collides_at(&self, brick: &Vec<Vec<bool>>, pos: &(i32, i32)) -> Option<bool>;
}

impl Chaimber for Vec<Vec<bool>> {

    fn repeats(&self) -> Option<Vec<Vec<bool>>> {
        for slice_size in (4..self.tower_height() / 2).rev() {
            let a = &self[self.tower_height() - slice_size..self.tower_height()];
            let b = &self[self.tower_height() - (slice_size * 2)..self.tower_height() - slice_size];
            if a == b {
                return Some(a.to_vec());
            }
        }

        None
    }

    fn play_brick(&mut self, brick: &Vec<Vec<bool>>, directions: &Vec<Direction>, direction_index: &mut usize) {
        let heighest_brick = self.tower_height();
        let free = self.len() - heighest_brick;

        self.extend(vec![
            vec![false; 7];
            (3 + brick.len())
                .checked_sub(free)
                .unwrap_or_default()
        ]);

        let mut brick_origin: (i32, i32) = (4, (heighest_brick + 3) as i32);
        loop {
            let direction = &directions[*direction_index % directions.len()];
            *direction_index += 1;

            let try_move_to = match direction {
                Direction::West => (brick_origin.0 + 1, brick_origin.1),
                Direction::East => (brick_origin.0 - 1, brick_origin.1),
                _ => panic!("No valid direction")
            };

            if let Some(collides) = self.collides_at(brick, &try_move_to) {
                if !collides {
                    brick_origin = try_move_to;
                }
            }

            let try_fall = (brick_origin.0, brick_origin.1 - 1);
            let collides_below = self.collides_at(brick, &try_fall);
            if collides_below.is_none() || collides_below.unwrap() {
                break;
            } else {
                brick_origin = try_fall;
            }
        }

        for (local_y, row) in brick.iter().rev().enumerate() {
            for (local_x, filled) in row.iter().enumerate().rev() {
                if !*filled {
                    continue;
                }

                let (chamber_x, chamber_y) = (
                    brick_origin.0 as usize - local_x,
                    brick_origin.1 as usize + local_y
                );

                *self.get_mut(chamber_y).and_then(|row| row.get_mut(chamber_x)).unwrap() = true;
            }
        }
    }

    fn tower_height(&self) -> usize {
        self.len() - 
            self.into_iter()
                .rev()
                .position(|row| row.contains(&true))
                .map_or(0, |p| p)
    }

    fn collides_at(&self, brick: &Vec<Vec<bool>>, (brick_x, brick_y): &(i32, i32)) -> Option<bool> {
        for (local_y, row) in brick.iter().rev().enumerate() {
            for (local_x, filled) in row.iter().enumerate().rev() {
                let (check_x, check_y) = (*brick_x - (local_x as i32), (*brick_y + (local_y as i32)));
                if check_x >= 0 && check_x < (self.get(0).map_or(0, |x| x.len() as i32)) && check_y >= 0 {
                    if *filled && self[check_y as usize][check_x as usize] {
                        return Some(true);
                    }
                } else {
                    return None;
                }
            }
        }

        Some(false)
    }
}

fn parse_input(input: String) -> Vec<Direction> {
    input.chars()
        .map(Direction::from)
        .collect_vec()
}

fn make_bricks<'a>() -> [Vec<Vec<bool>>; 5] {
    let minus: Vec<Vec<bool>> = vec![vec![true, true, true, true]];
    let plus: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![true, true, true],
        vec![false, true, false],
    ];
    let corner: Vec<Vec<bool>> = vec![
        vec![false, false, true],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let bar: Vec<Vec<bool>> = vec![vec![true], vec![true], vec![true], vec![true]];
    let square: Vec<Vec<bool>> = vec![vec![true, true], vec![true, true]];

    [minus, plus, corner, bar, square]
}