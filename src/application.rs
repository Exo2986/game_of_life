use opengl_graphics::GlGraphics;
use graphics::types::Color;
use piston::{RenderArgs, UpdateArgs, Button, MouseButton};
use graphics::color::*;
use crate::automata::grid::Grid;
use crate::automata::cell::Cell;

pub struct App {
    pub gl: GlGraphics,
    pub grid: Grid,
    pub update_time: f64,
    pub mouse_pos: [f64;2],
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let mut rect = Rectangle::new_border(BLACK, 1.0);

        let grid = &self.grid;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);

            for x in 0..grid.size {
                for y in 0..grid.size {
                    let transform = c.transform.trans(x as f64*50.0, y as f64*50.0);

                    rect.color =
                        if grid.cells.get(&[x, y]).unwrap().state == true {
                            BLACK
                        } else {
                            WHITE
                        };

                    rect.draw(square, &c.draw_state, transform, gl);
                }
            }
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.update_time - 1.0 > 0.0 {
            self.update_time = 0.0;
            let grid: &mut Grid = &mut self.grid;

            for x in 0..grid.size {
                for y in 0..grid.size {
                    let mut cell: Cell = grid.cells.get(&[x,y]).expect("Invalid cell!").clone();

                    cell.state = cell.nextState;

                    grid.cells.insert([x,y], cell);
                }
            }

            for x in 0..grid.size {
                for y in 0..grid.size{
                    let mut cell: Cell = grid.cells.get(&[x,y]).expect("Invalid cell!").clone();
                    let neighbors = grid.get_neighbors(&cell);
                    let mut score = 0;
                    println!("{}, {}, ({})", cell.x, cell.y, score);

                    for i in 0..neighbors.len() {
                        score += neighbors[i].state as i32;
                    }

                    match cell.state {
                        true => { //alive
                            if score != 2 && score != 3 {
                                cell.nextState = false;
                                println!("AAAA");
                            }
                        }
                        false => { //dead
                            if score == 3 {
                                cell.nextState = true;
                            }
                        }
                    }

                    grid.cells.insert([x, y], cell);
                }
            }
        }

        self.update_time+=args.dt;
    }

    pub fn inputs(&mut self, args: &Button, press_event: bool) { //press_event false if release
        if Button::Mouse(MouseButton::Left) == *args {
            let x: i32 = (self.mouse_pos[0]/50.0) as i32;
            let y: i32 = (self.mouse_pos[1]/50.0) as i32;
            let mut cell: Cell = self.grid.cells.get(&[x,y]).expect("Invalid cell!").clone();

            cell.state = true;
            cell.nextState = true;

            self.grid.cells.insert([x,y], cell);

        }
    }

    pub fn mouse_cursor(&mut self, mouse_pos: &[f64; 2]) {
        self.mouse_pos = mouse_pos.clone();
    }
}