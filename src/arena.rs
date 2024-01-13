use crate::{
    cell::{Cell, SquareCell},
    direction::{Directions, SquareDirections},
};
use rand::Rng;

pub trait Arena<C: Cell, D: Directions> {
    fn get_random_cell(&self) -> C;
    fn get_neighbor(&self, cell: &C, direction: &D) -> Option<C>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct RectangleArena {
    cells: Vec<Vec<SquareCell>>,
}

impl Arena<SquareCell, SquareDirections> for RectangleArena {
    fn get_random_cell(&self) -> SquareCell {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.cells.len());
        let col = rng.gen_range(0..self.cells[row].len());
        //todo: need to actually make it random
        self.cells[row][col]
    }

    fn get_neighbor(&self, cell: &SquareCell, direction: &SquareDirections) -> Option<SquareCell> {
        let (row, col) = cell.get_neighbor_coordinates(direction);
        if row < 0 || row as usize >= self.cells.len() {
            return None;
        }
        if col < 0 || col as usize >= self.cells[row as usize].len() {
            return None;
        }
        Some(self.cells[row as usize][col as usize])
    }
}

impl RectangleArena {
    pub fn new(rows: u8, columns: u8) -> Self {
        let mut arena = RectangleArena { cells: vec![] };

        for i in 0..rows {
            let mut row: Vec<SquareCell> = vec![];
            for j in 0..columns {
                row.push(SquareCell::new(i, j))
            }
            arena.cells.push(row);
        }
        arena
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_new_arena() {
        let arena1by1 = RectangleArena::new(1, 1);
        let cell1 = arena1by1.get_random_cell();

        assert!(arena1by1
            .get_neighbor(&cell1, &SquareDirections::North)
            .is_none());
        assert!(arena1by1
            .get_neighbor(&cell1, &SquareDirections::South)
            .is_none());
        assert!(arena1by1
            .get_neighbor(&cell1, &SquareDirections::East)
            .is_none());
        assert!(arena1by1
            .get_neighbor(&cell1, &SquareDirections::West)
            .is_none());

        let arena2by3 = RectangleArena::new(3, 2);

        let cell1a = get_futhest_neighbor(
            &arena2by3,
            &get_futhest_neighbor(
                &arena2by3,
                &arena2by3.get_random_cell(),
                &SquareDirections::North,
                5,
            ),
            &SquareDirections::East,
            5,
        );

        let cell1b = arena2by3
            .get_neighbor(&cell1a, &SquareDirections::South)
            .unwrap();
        let cell1c: SquareCell = arena2by3
            .get_neighbor(&cell1b, &SquareDirections::South)
            .unwrap();
        let cell2a = arena2by3
            .get_neighbor(&cell1a, &SquareDirections::West)
            .unwrap();
        let cell2b = arena2by3
            .get_neighbor(&cell1b, &SquareDirections::West)
            .unwrap();
        let cell2c = arena2by3
            .get_neighbor(&cell1c, &SquareDirections::West)
            .unwrap();

        check_neighbors(&arena2by3, &cell1a, None, Some(cell1b), None, Some(cell2a));
        check_neighbors(
            &arena2by3,
            &cell1b,
            Some(cell1a),
            Some(cell1c),
            None,
            Some(cell2b),
        );
        check_neighbors(&arena2by3, &cell1c, Some(cell1b), None, None, Some(cell2c));
        check_neighbors(&arena2by3, &cell2a, None, Some(cell2b), Some(cell1a), None);
        check_neighbors(
            &arena2by3,
            &cell2b,
            Some(cell2a),
            Some(cell2c),
            Some(cell1b),
            None,
        );
        check_neighbors(&arena2by3, &cell2c, Some(cell2b), None, Some(cell1c), None);
    }

    pub fn get_futhest_neighbor(
        arena: &RectangleArena,
        starting_cell: &SquareCell,
        direction: &SquareDirections,
        limit: u8,
    ) -> SquareCell {
        let mut cell = starting_cell.clone();
        let mut count = 0;
        while count < limit {
            match arena.get_neighbor(&cell, direction) {
                None => return cell,
                Some(neighbor) => cell = neighbor,
            }
            count += 1;
        }

        cell
    }

    pub fn check_neighbors(
        arena: &RectangleArena,
        cell: &SquareCell,
        north_neighbor: Option<SquareCell>,
        south_neighbor: Option<SquareCell>,
        east_neighbor: Option<SquareCell>,
        west_neighbor: Option<SquareCell>,
    ) {
        assert_eq!(
            arena.get_neighbor(&cell, &SquareDirections::North),
            north_neighbor
        );
        assert_eq!(
            arena.get_neighbor(&cell, &SquareDirections::South),
            south_neighbor
        );
        assert_eq!(
            arena.get_neighbor(&cell, &SquareDirections::East),
            east_neighbor
        );
        assert_eq!(
            arena.get_neighbor(&cell, &SquareDirections::West),
            west_neighbor
        );
    }
}
