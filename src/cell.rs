use crate::direction::SquareDirections;

pub trait Cell {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SquareCell {
    row: u8,
    column: u8,
}

impl SquareCell {
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }

    pub fn get_neighbor_coordinates(&self, direction: &SquareDirections) -> (i32, i32) {
        match direction {
            SquareDirections::North => (self.row as i32 - 1, self.column as i32),
            SquareDirections::South => (self.row as i32 + 1 as i32, self.column as i32),
            SquareDirections::East => (self.row as i32, self.column as i32 - 1),
            SquareDirections::West => (self.row as i32, self.column as i32 + 1),
        }
    }
}

impl Cell for SquareCell {}
