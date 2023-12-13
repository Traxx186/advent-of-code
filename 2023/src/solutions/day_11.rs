use aoc_common::{Solution, point::Point2D, load};
use itertools::Itertools;

pub struct Day11;

type Galaxy = Point2D<usize>;

impl Solution for Day11 {
    fn name(&self) -> String {
        "Cosmic Expansion".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_11");
        let galaxies = parse_galaxies(input, 2);

        let path_length = galaxies.iter()
            .enumerate()
            .fold(0, |mut acc, (i, galaxy)| {
                for next_galaxy in &galaxies[i + 1..] {
                    acc += manhattan_distance(galaxy, next_galaxy);
                }

                acc
            });

        path_length.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_11");
        let galaxies = parse_galaxies(input, 1_000_000);


        let path_length = galaxies.iter()
            .enumerate()
            .fold(0, |mut acc, (i, galaxy)| {
                for next_galaxy in &galaxies[i + 1..] {
                    acc += manhattan_distance(galaxy, next_galaxy);
                }

                acc
            });

        path_length.to_string()
    }
}

fn parse_grid(input: String) -> (Vec<Vec<char>>, Vec<Galaxy>) {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut galaxies: Vec<Point2D<usize>> = vec![];
    
    for (x, line) in input.lines().enumerate() {
        for (y, character) in line.chars().enumerate() {
            if character == '#' {
                grid[x][y] = character;
                galaxies.push(Point2D { x, y })
            }
        }
    }

    (grid, galaxies)
}

fn parse_galaxies(input: String, expansion: usize) -> Vec<Galaxy> {
    let (grid, mut galaxies) = parse_grid(input);

    let empty_rows = grid.clone()
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            if line.iter().any(|&c| c == '#') {
                None
            } else {
                Some(row)
            }
        })
        .collect_vec();

    let mut empty_columns = vec![];
    for column in 0..grid[0].len() {
        let mut empty_column = true;
        for row in 0..grid.len() {
            if grid[row][column] == '#' {
                empty_column = false;
                break;
            }
        }

        if empty_column {
            empty_columns.push(column);
        }
    }

    for galaxy in galaxies.iter_mut() {
        let (row, column) = (galaxy.x, galaxy.y);
        let rows_added = empty_rows.iter().filter(|&r| r < &row).count() * (expansion - 1);
        let columns_added = empty_columns.iter().filter(|&c| c < &column).count() * (expansion - 1);

        *galaxy = Point2D { x: row + rows_added, y: column + columns_added };
    }

    galaxies
}

fn manhattan_distance(a: &Galaxy, b: &Galaxy) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}