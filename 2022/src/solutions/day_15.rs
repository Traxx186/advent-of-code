use std::cmp::max;

use aoc_common::{Solution, load, point::Point2D};
use regex::Regex;

pub struct Day15 {}

const MAX_Y: i32 = 4_000_000;
const Y: i32 = 2_000_000;

impl Solution for Day15 {

    fn name(&self) -> String {
        "Beacon Exclusion Zone".to_owned()
    }

    fn part_1(&self) -> String {
        let coverages = parse_input(load("day_15"));
        let disjoint_intervals = process_row(Y, &coverages);

        let mut counted_beacons: Vec<i32> = vec![];
        let mut count = 0;

        disjoint_intervals.iter().for_each(|i| { count += i.to - i.from + 1 });

        for coverage in coverages {
            let beacon = coverage.beacon;
            if (beacon.y == Y) && !counted_beacons.contains(&beacon.x) {
                count -= 1;
                counted_beacons.push(beacon.x);
            }
        }

        count.to_string()
    }

    fn part_2(&self) -> String {
        let coverages = parse_input(load("day_15"));
        let mut position = Point2D::<i32> { x: 0, y: 0} ;

        for i in 0..MAX_Y {
            let disjoint_intervals = process_row(i, &coverages);
            if disjoint_intervals.len() <= 1 {
                continue;
            }

            position.y = i;
            position.x = disjoint_intervals[0].to + 1;
        }
        
        let freq: i64 = ((position.x as i64) * MAX_Y as i64) + (position.y as i64);
        freq.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coverage {
    sensor: Point2D<i32>,
    beacon: Point2D<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Interval {
    from: i32,
    to: i32,
}

fn parse_input(input: String) -> Vec<Coverage> {
    let mut out = vec![];
    let regex = Regex::new(r"[\-]?\d+").unwrap();

    for line in input.lines() {
        let mut result = regex.find_iter(line);
        
        let sensor_x = result.next().unwrap().as_str();
        let sensor_y = result.next().unwrap().as_str();
        let beacon_x = result.next().unwrap().as_str();
        let beacon_y = result.next().unwrap().as_str();

        let sensor = Point2D::<i32> { x: sensor_x.parse().unwrap(), y: sensor_y.parse().unwrap() };
        let beacon = Point2D::<i32> { x: beacon_x.parse().unwrap(), y: beacon_y.parse().unwrap() };

        out.push(Coverage { sensor, beacon });
    }

    out
}

fn process_row(row: i32, coverages: &Vec<Coverage>) -> Vec<Interval> {
    let mut intervals: Vec<Interval> = vec![];
    let mut disjoint_intervals: Vec<Interval> = vec![];

    for coverage in coverages {
        let dist = (coverage.sensor.x - coverage.beacon.x).abs() + (coverage.sensor.y - coverage.beacon.y).abs();
        let row_dist = (coverage.sensor.y - row).abs();
        
        if row_dist >= dist {
            continue;
        }

        intervals.push(Interval{
                from: coverage.sensor.x - (dist - row_dist),
                to: coverage.sensor.x + (dist - row_dist),
        });    
    }

    intervals.sort();
    for interval in intervals {
        if disjoint_intervals.is_empty() {
            disjoint_intervals.push(interval);
            continue;
        }

        let last_index = disjoint_intervals.len() - 1;
        let last_interval = disjoint_intervals.get_mut(last_index).unwrap();
        if interval.from <= last_interval.to {
            last_interval.to = max(last_interval.to, interval.to);
            continue;
        }

        disjoint_intervals.push(interval);    
    }

    disjoint_intervals
}
