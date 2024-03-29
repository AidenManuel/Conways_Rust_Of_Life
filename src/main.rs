extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const ROWS: usize = 240;
const COLS: usize = 320;
const SIZE: usize = ROWS * COLS;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    state: [bool; SIZE]
}

impl App {
    /////////
    // RENDER FUNCTION
    ///////

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Defining all necessary constants

        const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        
        let mut colour = BLACK;

        // Variable Declarations

        let state: [bool; SIZE] = self.state;

            // iterators
        let mut x = 0.0;
        let mut y = 0.0;

        /////////
        // DRAW FUNCTION
        ///////

        while y < (ROWS as f64){
            while x < (COLS as f64){
                let square = rectangle::square(x, y, 1.0);
                
                self.gl.draw(args.viewport(), |c, gl| {
                    
                    colour = BLACK;

                    if state[(x as usize) + (y as usize) * COLS]{
                        colour = WHITE;
                    }

                    // Clear the screen.
                    // clear(WHITE, gl);

                    let transform = c
                        .transform;

                    rectangle(colour, square, transform, gl);
                });
                x += 1.0;
            }
            x = 0.0;
            y += 1.0;
        }
    }

    
    /////////
    // UPDATE FUNCTION
    ///////
    
    fn update(&mut self, _args: &UpdateArgs) {
        let mut i = 0;

        // Populating State Array Randomly

            // state array will determine whether a cell is "alive" or "dead"
        while i < SIZE {
            self.state[i] = !self.state[i];
            i = i + 1;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [COLS as f64, ROWS as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Creating and Populating State Array Randomly
            let mut state: [bool; SIZE] = [false; SIZE];
            let mut i = 0;

            // state array will determine whether a cell is "alive" or "dead"
            while i < SIZE {
                state[i] = rand::random();
                i = i + 1;
            }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: state 
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
