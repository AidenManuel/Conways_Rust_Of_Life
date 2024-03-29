////////////////////////////////////////////////////////////////////////
/// 
///  #####                                           #                       
/// #     #   ##   #    # ######     ####  ######    #       # ###### ###### 
/// #        #  #  ##  ## #         #    # #         #       # #      #      
/// #  #### #    # # ## # #####     #    # #####     #       # #####  #####  
/// #     # ###### #    # #         #    # #         #       # #      #      
/// #     # #    # #    # #         #    # #         #       # #      #      
///  #####  #    # #    # ######     ####  #         ####### # #      ###### 
///
///////////////////////////////////////////////////////////////////////
///
///  My first attempt at a Game of Life, and also my first time doing
///  something in Rust (so go easy on me). Using Piston and OpenGL for
///  rendering, based on a getting started tutorial from the Piston
///  Github.
///
///////////////////////////////////////////////////////////////////////
/// 
///  Aiden Manuel - March 29th, 2024 - 3rd Year Computer Science Student
///                                                                     
///////////////////////////////////////////////////////////////////////

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate chrono;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::GenericEvent;

const HEIGHT: usize = 600;
const WIDTH: usize = 900;
const SCALE: usize = 10;
const ROWS: usize = HEIGHT / SCALE;
const COLS: usize = WIDTH / SCALE;
const SIZE: usize = (ROWS) * (COLS);

pub struct App { 
    // OpenGL drawing backend.
    gl: GlGraphics,
    state: [bool; SIZE],
    cursor_pos: [f64; 2],
    paused: bool
}

impl App {
    /////////
    /// RENDER FUNCTION
    ///////

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Defining all necessary constants

        const WHITE: [f32; 4] = [0.9, 0.9, 0.85, 1.0];
        const BLACK: [f32; 4] = [0.6, 0.5, 0.52, 1.0];
        
        let mut colour = BLACK;

        // Variable Declarations

        let state: [bool; SIZE] = self.state;

            // iterators
        let mut x = 0.0;
        let mut y = 0.0;

        /////////
        // DRAW FUNCTION
        ///////

        while y < ((ROWS) as f64){
            while x < ((COLS) as f64){

                // We draw each cell as a square, which is a data structure
                // with 4 floating point values.
                let square = rectangle::square(x * SCALE as f64, y * SCALE as f64, SCALE as f64);
                
                // OpenGL is used for rendering it to the screen.
                self.gl.draw(args.viewport(), |c, gl| {
                    
                    // Colour white if DEAD and black if ALIVE
                    colour = WHITE;

                    if state[(x as usize) + (y as usize) * (COLS)]{
                        colour = BLACK;
                    }

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
    /// UPDATE FUNCTION
    ///////
    
    fn update(&mut self, _args: &UpdateArgs) {
        if !self.paused {
            let mut i = 0;
            let mut neighbour = 0;
            let previous_state: [bool; SIZE] = self.state;

            // For each pixel in the scene...
            while i < SIZE {

                // Check if all the neighbours are alive or dead...
                if previous_state[(SIZE + i - 1 - COLS) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i - COLS) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i + 1 - COLS) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i - 1) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i + 1) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i - 1 + COLS) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i + COLS) % SIZE] {neighbour += 1;}
                if previous_state[(SIZE + i + 1 + COLS) % SIZE] {neighbour += 1;}

                // Based on current state, change to new state!
                if previous_state[i] {
                    if neighbour < 2 || neighbour > 3 {
                        self.state[i] = !previous_state[i];
                    }
                } else if neighbour == 3 {
                    self.state[i] = !previous_state[i];
                } else {
                    self.state[i] = previous_state[i];
                }

                // Don't forget to update your variables ;)
                i += 1;
                neighbour = 0;
            }
        }
    }

    /////////
    /// EVENT FUNCTION
    ////////
    
    fn event<E: GenericEvent>(&mut self, pos: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        // Mouse Function Added!
        // Left Click to change the flip the state of a cell
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= WIDTH as f64 && y >= 0.0 && y <= HEIGHT as f64 {
                // Compute the cell position.
                let cell_x = (x / SCALE as f64) as usize;
                let cell_y = (y / SCALE as f64) as usize;
                // Flip the state of that cell
                self.state[(cell_x as usize) + (cell_y as usize) * (COLS)] = !self.state[(cell_x as usize) + (cell_y as usize) * (COLS)];
            }
        }
        // Key Functions Added!
        // Space:   pause the game
        // C:       cull all living cells
        // R:       create a random starting board
        if let Some(Button::Keyboard(key)) = e.press_args() {
                let mut i = 0;
                match key {
                    Key::Space => self.paused = !self.paused,
                    Key::C => self.state = [false; SIZE],
                    Key::R => while i < SIZE { self.state[i] = rand::random(); i = i + 1; },
                    _ => {}
            }
        }
    }
}

///////////////////////////////
// Most of this main method was not programmed by me. It comes
// from a Piston tutorial, which is the crate I'm using to update
// the do the graphics. See example from repo here:
// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
///////////////////////////////

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [WIDTH as f64, HEIGHT as f64])
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
        state: state,
        cursor_pos: [0.0, 0.0],
        paused: false,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        app.event([0.0, 0.0], &e);

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
