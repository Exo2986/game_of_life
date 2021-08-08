#[derive(Clone)]
pub enum Direction {
    X, Y,
}

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub state: bool,
    pub nextState: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32, size: u32) -> Cell {
        Cell {
            x,
            y,
            size,
            state: if y == 2 {
                true
            } else {
                false
            },
            nextState: if y == 2 {
                true
            } else {
                false
            },
        }
    }

    pub fn sub_one(& self, direction: Direction) -> u32 {
        match direction {
            Direction::X => {
                if self.x-1 < 0 {
                    self.size-1
                } else {
                    self.x-1
                }
            }
            Direction::Y => {
                if self.y-1 < 0 {
                    self.size-1
                } else {
                    self.y-1
                }
            }
        }
    }

    pub fn add_one(& self, direction: Direction) -> u32 {
        match direction {
            Direction::X => {
                if self.x+1 >= self.size {
                    0
                } else {
                    self.x+1
                }
            }
            Direction::Y => {
                if self.y+1 >= self.size {
                    0
                } else {
                    self.y+1
                }
            }
        }
    }
}