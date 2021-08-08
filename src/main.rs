use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, MouseCursorEvent};
use piston::window::WindowSettings;
use graphics::color::{WHITE, RED, BLUE, BLACK, GREEN};
use piston::{PressEvent, Button, MouseButton};
use graphics::types::Color;
use crate::automata::grid::Grid;

mod application;

mod automata;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning square", [200,200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = application::App {
        gl: GlGraphics::new(opengl),
        grid: Grid::new(15),
        update_time: 0.0,
        mouse_pos: [0.0, 0.0]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        
        if let Some(args) = e.press_args() {
            app.inputs(&args, true);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_cursor(&args)
        }
    }
}
