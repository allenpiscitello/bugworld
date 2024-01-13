use crate::{cell::SquareCell, direction::SquareDirections};

pub trait Arena {
    fn get_random_cell(&self) -> SquareCell;
    fn get_neighbor(&self, cell: &SquareCell, direction: &SquareDirections) -> Option<SquareCell>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct RectangleArena {}

impl Arena for RectangleArena {
    fn get_random_cell(&self) -> SquareCell {
        todo!()
    }

    fn get_neighbor(&self, cell: &SquareCell, direction: &SquareDirections) -> Option<SquareCell> {
        todo!()
    }
}

impl RectangleArena {
    pub fn new(rows: u8, columns: u8) -> Self {
        todo!()
    }
}
