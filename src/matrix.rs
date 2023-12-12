use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::direction::Direction;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T: Copy> {
    pub value: T,
    pub column: usize,
    pub row: usize
}

impl<T: Copy> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.row == other.row
    }
}

impl<T: Copy> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.column.hash(state);
        self.row.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<T: Copy> {
    tiles: Vec<Vec<T>>,
    width: usize,
    height: usize
}

impl<T: Copy> Matrix<T> {
    
    pub fn new(tiles: Vec<Vec<T>>) -> Self {
        let width = tiles[0].len();
        let height = tiles.len();
    
        Matrix { tiles, width, height }
    }

    pub fn items(&self) -> impl Iterator<Item = Cell<T>> + '_ {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(row, column)| self.get_cell(row, column))
    }

    pub fn get(&self, row: usize, column: usize) -> Option<T> {
        self.tiles.get(row)
            .and_then(|r| r.get(column).copied())
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Cell<T> {
        self.get(row, column)
            .map(|t| Cell { value: t, row, column})
            .unwrap()
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.tiles.get_mut(row)
            .and_then(|tile| tile.get_mut(column))
    }

    pub fn neighbour(&self, cell: &Cell<T>, direction: &Direction) -> Option<Cell<T>> {
        match direction {
            Direction::North => {
                let column = cell.column;
                let row = cell.row.checked_sub(1)?;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::NorthEast => {
                let column = cell.column + 1;
                let row = cell.row.checked_sub(1)?;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::East => {
                let row = cell.row;
                let column = cell.column + 1;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::SouthEast => {
                let row = cell.row + 1;
                let column = cell.column + 1;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::South => {
                let row = cell.row + 1;
                let column = cell.column;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::SouthWest => {
                let row = cell.row + 1;
                let column = cell.column.checked_sub(1)?;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::West => {
                let row = cell.row;
                let column = cell.column.checked_sub(1)?;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
            Direction::NorthWest => {
                let row = cell.row.checked_sub(1)?;
                let column = cell.column.checked_sub(1)?;
                let value = self.get(row, column)?;

                Some(Cell { value, column, row })
            },
        }
    }
}