use aoc_common::{
    math::median,
    load, 
    Solution
};

pub struct Day07 {}

impl Solution for Day07 {
    fn name(&self) -> String {
        "The Treachery of Whales".to_owned()
    }

    fn part_1(&self) -> String {
        let data = load("day_07");
        let mut positions = data.split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let middle = median( &mut positions);
        let mut sum = 0;

        for position in &positions {
            if position == &middle {
                continue;
            }

            if position > &middle {
                sum += position - middle
            } else  {
                sum += middle - position
            }
        }

        sum.to_string()
    }

    fn part_2(&self) -> String {
        let data = load("day_07");
        let positions = data.split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let min = positions.iter().min().unwrap();
        let max = positions.iter().max().unwrap();

        let mut min_fuel = u32::MAX;
        for i in *min..=*max {
            let cost = move_crabs(&positions, i);
            if cost < min_fuel {
                min_fuel = cost;
            }
        }

        min_fuel.to_string()
    }
}

fn move_crabs(crabs: &[u32], to: u32) -> u32 {
    let mut cost = 0;
    for crab in crabs {
        cost += (0..=(*crab as i32 - to as i32 ).abs()).sum::<i32>();
    }

    cost as u32
}