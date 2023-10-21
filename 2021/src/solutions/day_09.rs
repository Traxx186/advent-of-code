use aoc_common::{load, Solution};

pub struct Day09 {}

impl Solution for Day09 {

    fn name(&self) -> String {
        "Smoke Basin".to_owned()
    }

    fn part_1(&self) -> String {
        let data = parse_input(load("day_09"));
        let lowest_points = data.iter()
            .enumerate()
            .flat_map(|(i, line)| {
                let input = &data;
                line.iter().enumerate().filter_map(move |(j, &h)| {
                    if (i == 0 || h < input[i - 1][j])
                        && (i == input.len() - 1 || h < input[i + 1][j])
                        && (j == 0 || h < input[i][j - 1])
                        && (j == line.len() - 1 || h< input[i][j + 1])
                    { return Some(h) }
                    None
                })
            }).collect::<Vec<u32>>();

        lowest_points.iter()
            .map(|x| *x + 1)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut data = parse_input(load("day_09"));
        let mut basins = Vec::new();

        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] < 9 {
                    basins.push(basin(&mut data, j, i));
                }
            }
        }

        basins.sort_unstable();
        basins.iter().rev().take(3).product::<u32>().to_string()
    }
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let digits = line.chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        out.push(digits);
    }

    out
}

fn basin(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    map[y][x] = 9;
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |inc, (x, y)| {
            if map.get(y).and_then(|l| l.get(x)).map(|&n| n < 9) == Some(true) {
                return inc + basin(map, x, y)
            }

            inc
        })
}