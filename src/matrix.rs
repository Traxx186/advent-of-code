use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::{direction::Direction, point::Point2D};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T: Copy> {
    pub value: T,
    pub coordinate: Point2D<usize>,
}

impl<T: Copy> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}

impl<T: Copy> Hash for Cell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
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
            .map(|t| Cell { value: t, coordinate: Point2D { x: row, y: column }})
            .unwrap()
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.tiles.get_mut(row)
            .and_then(|tile| tile.get_mut(column))
    }

    pub fn neighbour(&self, cell: &Cell<T>, direction: &Direction) -> Option<Cell<T>> {
        match direction {
            Direction::North => {
                let column = cell.coordinate.y;
                let row = cell.coordinate.x.checked_sub(1)?;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::NorthEast => {
                let column = cell.coordinate.y + 1;
                let row = cell.coordinate.x.checked_sub(1)?;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::East => {
                let row = cell.coordinate.x;
                let column = cell.coordinate.y + 1;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::SouthEast => {
                let row = cell.coordinate.x + 1;
                let column = cell.coordinate.y + 1;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::South => {
                let row = cell.coordinate.x + 1;
                let column = cell.coordinate.y;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::SouthWest => {
                let row = cell.coordinate.x + 1;
                let column = cell.coordinate.y.checked_sub(1)?;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::West => {
                let row = cell.coordinate.x;
                let column = cell.coordinate.y.checked_sub(1)?;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
            Direction::NorthWest => {
                let row = cell.coordinate.x.checked_sub(1)?;
                let column = cell.coordinate.y.checked_sub(1)?;
                let value = self.get(row, column)?;
                let coordinate = Point2D { x: row, y: column };

                Some(Cell { value, coordinate })
            },
        }
    }
}