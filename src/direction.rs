pub trait Directions {}

pub enum SquareDirections {
    North,
    South,
    East,
    West,
}

impl Directions for SquareDirections {}
