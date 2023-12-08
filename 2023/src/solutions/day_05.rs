use std::{collections::HashMap, ops::Range};

use aoc_common::{load, Solution};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Map {
    source: String,
    destination: String,
    ranges: Vec<MapRange>,
}

#[derive(Clone, Debug)]
pub struct MapRange {
    range: Range<isize>,
    des_offset: isize,
}

impl Into<Map> for Vec<&String> {
    fn into(self) -> Map {
        let name_line = self.first().unwrap().strip_suffix(" map:").unwrap();

        let names: (&str, &str) = name_line.trim().split("-to-").collect_tuple().unwrap();

        let mut ranges: Vec<MapRange> = vec![];
        for line in self.iter().skip(1) {
            let (dest, src, len) = line
                .split_whitespace()
                .map(|item| item.parse::<isize>().unwrap())
                .collect_tuple::<(isize, isize, isize)>()
                .unwrap();

            let range = Range {
                start: src,
                end: src + len,
            };

            ranges.push(MapRange {
                range,
                des_offset: dest - src,
            });
        }

        Map {
            source: names.0.to_owned(),
            destination: names.1.to_owned(),
            ranges,
        }
    }
}

impl Map {
    fn convert(&self, source: isize) -> isize {
        let mut destination = source;
        for map_range in &self.ranges {
            if map_range.range.contains(&source) {
                destination = source + map_range.des_offset;
            }
        }

        destination
    }
}

#[derive(Clone, Debug)]
pub struct Almanac {
    seeds: Vec<isize>,
    maps: Vec<Map>,
}

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> String {
        "If You Give A Seed A Fertilizer".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_05");
        let almanac = parse_input(input);
        let mut min_location = isize::MAX;

        let sources: HashMap<String, &Map> =
            almanac.maps.iter().map(|m| (m.source.clone(), m)).collect();

        for seed in almanac.seeds {
            let location = find_location(&sources, seed);
            if location < min_location {
                min_location = location
            }
        }

        min_location.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_05");
        let mut almanac = parse_input(input);
        let mut min_location = isize::MAX;

        let mut new_seeds: Vec<isize> = vec![];
        for chunk in almanac.seeds.chunks(2) {
            for seed in chunk[0]..chunk[0] + chunk[1] {
                new_seeds.push(seed);
            }
        }

        almanac.seeds = new_seeds;

        let sources: HashMap<String, &Map> = almanac.maps.iter()
            .map(|m| (m.source.clone(), m))
            .collect();

        let progress_bar = ProgressBar::new(almanac.seeds.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
        );

        for seed in almanac.seeds {
            let location = find_location(&sources, seed);
            if location < min_location {
                min_location = location

            }
            progress_bar.inc(1);
        }

        progress_bar.finish_and_clear();

        min_location.to_string()
    }
}

fn parse_input(input: String) -> Almanac {
    let lines = input
        .lines()
        .map(|line| line.trim().to_string())
        .collect_vec();

    let maps = parse_maps(&lines);
    let seeds = parse_seeds(&lines);

    Almanac { maps, seeds }
}

fn parse_maps(lines: &Vec<String>) -> Vec<Map> {
    let mut maps = vec![];
    let mut chunks: Vec<&String> = vec![];

    for line in lines.iter().skip(2) {
        if line.is_empty() {
            maps.push(chunks.clone().into());
            chunks.clear();
        } else {
            chunks.push(line);
        }
    }

    if !chunks.is_empty() {
        maps.push(chunks.clone().into());
    }

    maps
}

fn parse_seeds(lines: &Vec<String>) -> Vec<isize> {
    lines[0]
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|split| split.parse::<isize>().unwrap())
        .collect_vec()
}

fn find_location(source_map: &HashMap<String, &Map>, seed: isize) -> isize {
    let mut current_map = source_map.get("seed").unwrap();
    let mut current_value = seed;

    while !current_map.destination.eq("location") {
        current_value = current_map.convert(current_value);
        current_map = source_map.get(&current_map.destination).unwrap();
    }

    current_map.convert(current_value)
}
