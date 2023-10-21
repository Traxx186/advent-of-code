use std::collections::HashMap;
use aoc_common::{load, Solution};

pub struct Day05 {}

impl Solution for Day05 {

    fn name(&self) -> String {
        "Hydrothermal Venture".to_owned()
    }

    fn part_1(&self) -> String {
        run(false)
    }

    fn part_2(&self) -> String {
        run(true)
    }
}

fn run(diagonal: bool) -> String {
    let segments = Segment::parse_input(load("day_05"), diagonal).unwrap();
    let mut map = HashMap::new();

    for segment in segments {
        let dump = segment.dump();
        for item in dump {
            if map.contains_key(&item) {
                *map.get_mut(&item).unwrap() += 1;
                continue;
            }
            map.insert(item, 1);
        }
    }

    map.iter().filter(|x| x.1 >= &2).count().to_string()
}

struct Segment {
    x1: u32,
    y1: u32,

    x2: u32,
    y2: u32
}

impl Segment {

    fn parse_input(input: String, diagonal: bool) -> Option<Vec<Segment>> {
        let mut out: Vec<Segment> = Vec::new();

        for line in input.lines()  {
            let mut segment = line.split(" -> ");

            let mut part_one = segment.next()?.split(',');
            let mut part_two = segment.next()?.split(',');

            let x1 = part_one.next()?.parse::<u32>().unwrap();
            let y1 = part_one.next()?.parse::<u32>().unwrap();
            let x2 = part_two.next()?.parse::<u32>().unwrap();
            let y2 = part_two.next()?.parse::<u32>().unwrap();

            if x1 != x2 && y1 != y2 && !diagonal {
                continue;
            }

            out.push(Segment { x1, y1, x2, y2 });
        }

        Some(out)
    }

    fn dump(&self) -> Vec<(u32, u32)> {
        let mut out = vec![(self.x1, self.y1)];
        let mut x = self.x1;
        let mut y = self.y1;

        while x != self.x2 || y != self.y2  {
            x -= (x > self.x2) as u32;
            x += (x < self.x2) as u32;
            y -= (y > self.y2) as u32;
            y += (y < self.y2) as u32;
            out.push((x, y));
        }

        out
    }
}