#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::West,
            '>' => Direction::East,
            _ => panic!("Not a valid character")
        }
    }
}