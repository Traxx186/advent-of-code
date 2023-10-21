#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Not a valid character")
        }
    }
}