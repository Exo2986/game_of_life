
pub enum Direction {
    X, Y,
}
#[derive(Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub state: bool,
    pub nextState: bool,
}

impl Cell {
    pub fn new(x: i32, y: i32, size: i32) -> Cell {
        Cell {
            x,
            y,
            size,
            state: false,
            nextState: false,
        }
    }

    pub fn sub_one(& self, direction: Direction) -> i32 {
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

    pub fn add_one(& self, direction: Direction) -> i32 {
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