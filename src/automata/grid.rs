use std::collections::HashMap;

use crate::automata::cell::{Cell, Direction};
use piston::input::keyboard::Key::Hash;

pub struct Grid{
    pub cells: HashMap<[i32; 2], Cell>,
    pub size: i32,
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

    pub fn get_neighbors(& self, cell: &Cell) -> Vec<&Cell> {
        let mut neighbors = Vec::new();
        let cells = &self.cells;
        neighbors.push(cells.get(&[cell.sub_one(Direction::X), cell.sub_one(Direction::Y)]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.sub_one(Direction::X), cell.y]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.sub_one(Direction::X), cell.add_one(Direction::Y)]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.add_one(Direction::X), cell.sub_one(Direction::Y)]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.add_one(Direction::X), cell.y]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.add_one(Direction::X), cell.add_one(Direction::Y)]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.x, cell.add_one(Direction::Y)]).expect("Missing neighbor!"));
        neighbors.push(cells.get(&[cell.x, cell.sub_one(Direction::Y)]).expect("Missing neighbor!"));

        neighbors
    }

    pub fn new(size: i32) -> Grid {
        let mut grid = Grid {
            cells: HashMap::new(),
            size,
        };
        grid.initialize_cells();
        grid
    }
}