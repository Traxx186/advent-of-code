use std::collections::{HashMap, HashSet};

use aoc_common::{Solution, load, direction::Direction, matrix::{Matrix, Cell}};
use itertools::Itertools;
use once_cell::sync::Lazy;

const CARDINAL: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

const ORDINAL: [Direction; 4] = [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest];

static PIPES: Lazy<HashMap<Pipe, Vec<(Direction, Direction)>>> = Lazy::new(|| HashMap::from_iter([
    (Pipe::Vertical, vec![(Direction::North, Direction::North), (Direction::South, Direction::South)]),
    (Pipe::Horizontal, vec![(Direction::East, Direction::East), (Direction::West, Direction::West)]),
    (Pipe::BendNE, vec![(Direction::South, Direction::East), (Direction::West, Direction::North)]),
    (Pipe::BendNW, vec![(Direction::South, Direction::West), (Direction::East, Direction::North)]),
    (Pipe::BendSW, vec![(Direction::East, Direction::South), (Direction::North, Direction::West)]),
    (Pipe::BendSE, vec![(Direction::North, Direction::East), (Direction::West, Direction::South)]),
]));

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> String {
        "Pipe Maze".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_10");
        let tiles: Vec<Vec<Pipe>> = input.lines()
            .map(|line| line.chars())
            .map(|chars| chars.map(|c| c.into()).collect_vec())
            .collect_vec();

        let mut grid = Matrix::new(tiles);

        find_loop(&mut grid)
            .map(|pipe| pipe.len() / 2)
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_10");
        let tiles: Vec<Vec<Pipe>> = input.lines()
            .map(|line| line.chars())
            .map(|chars| chars.map(|c| c.into()).collect_vec())
            .collect_vec();

        let mut grid = Matrix::new(tiles);
        let pipes: HashSet<Cell<Pipe>> = find_loop(&mut grid).map(HashSet::from_iter).unwrap();
        
        let tiles_sum = grid.items()
            .filter(|cell| {
                if pipes.contains(cell) {
                    return false;
                }

                let row = cell.row;
                let mut crosses = 0;

                for column in 0..cell.column {
                    let next = grid.get_cell(row, column);
                    if vec![Pipe::Vertical, Pipe::BendSE, Pipe::BendSW].contains(&next.value) && pipes.contains(&next) {
                        crosses += 1;
                    }
                }

                crosses % 2 != 0
            })
            .count();

        tiles_sum.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Start,
    Ground
}

impl Into<Pipe> for char {
    fn into(self) -> Pipe {
        match self {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::BendNE,
            'J' => Pipe::BendNW,
            '7' => Pipe::BendSW,
            'F' => Pipe::BendSE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid character")
        }
    }
}

fn find_loop(grid: &mut Matrix<Pipe>) -> Option<Vec<Cell<Pipe>>> {
    let items = grid.items().collect_vec();

    items.iter()
        .find(|c| c.value == Pipe::Start)
        .and_then(|start_cell| {
            CARDINAL.iter()
                .find_map(|direction| try_loop_from_start(grid, start_cell.clone(), *direction))
        })
}

fn try_loop_from_start(grid: &mut Matrix<Pipe>, start: Cell<Pipe>, start_dir: Direction) -> Option<Vec<Cell<Pipe>>> {
    let mut visited = vec![];

    let mut current_direction = start_dir;
    let mut current_cell = grid.neighbour(&start, &start_dir);

    loop {
        match current_cell {
            Some(cell) => {
                visited.push(cell);

                if cell.value == Pipe::Start {
                    let from = current_direction;
                    let to = start_dir;

                    let pipe_type = PIPES.iter()
                        .find(|pipe| pipe.1.iter().any(|&p| p == (from, to)))
                        .map(|pipe| *pipe.0)
                        .unwrap();

                    let new_cell = grid.get_mut(cell.row, cell.column).unwrap();
                    *new_cell = pipe_type;
                    
                    break;
                }

                let next = PIPES.get(&cell.value)
                    .and_then(|pipe| pipe.iter().find(|y| y.0 == current_direction).map(|y| y.1))
                    .map(|next_dir| (next_dir, grid.neighbour(&cell, &next_dir)));

                match next {
                    Some((direction, cell)) => {
                        current_direction = direction;
                        current_cell = cell;
                    },
                    None => return None,
                }
            },
            None => return None,
        }
    }

    Some(visited)
}