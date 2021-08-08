use std::collections::HashMap;

use crate::automata::cell::{Cell, Direction};

pub struct Grid{
    pub cells: &'static mut HashMap<[u32; 2], Cell>,
    pub size: u32,
}

impl Grid {
    fn initialize_cells(&mut self) {
        self.cells.clear();
        for column in 0..self.size {
            for row in 0..self.size {
                self.cells.insert([column, row], Cell::new(column, row, self.size));
            }
        }
    }

    pub fn get_neighbors(&mut self, cell: &Cell) -> Vec<&Cell> {
        let mut neighbors = Vec::new();
        let cells = &self.cells;
        neighbors[0] = cells.get(&[cell.sub_one(Direction::X), cell.sub_one(Direction::Y)]).expect("Missing neighbor!");
        neighbors[1] = cells.get(&[cell.sub_one(Direction::X), cell.y]).expect("Missing neighbor!");
        neighbors[2] = cells.get(&[cell.sub_one(Direction::X), cell.add_one(Direction::Y)]).expect("Missing neighbor!");
        neighbors[3] = cells.get(&[cell.add_one(Direction::X), cell.sub_one(Direction::Y)]).expect("Missing neighbor!");
        neighbors[4] = cells.get(&[cell.add_one(Direction::X), cell.y]).expect("Missing neighbor!");
        neighbors[5] = cells.get(&[cell.add_one(Direction::X), cell.add_one(Direction::Y)]).expect("Missing neighbor!");
        neighbors[6] = cells.get(&[cell.x, cell.add_one(Direction::Y)]).expect("Missing neighbor!");
        neighbors[7] = cells.get(&[cell.x, cell.sub_one(Direction::Y)]).expect("Missing neighbor!");

        neighbors
    }

    pub fn new(size: u32) -> Grid {
        let mut cells= &mut HashMap::new();
        let mut grid = Grid {
            cells,
            size,
        };
        grid.initialize_cells();
        grid
    }
}