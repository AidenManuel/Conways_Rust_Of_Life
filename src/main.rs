extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rows: i32,
    cols: i32,
    x: usize,
    y: usize
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Defining all necessary constants

        const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        const ROWS: usize = 200;
        const COLS: usize = 200;
        const SIZE: i32 = 200;

        self.rows = COLS as i32;
        self.cols = ROWS as i32;

        // Grids to store x and y coordinates of boxes

        let mut gridx: [f64; ROWS] = [0.0; ROWS];
        let mut gridy: [f64; COLS] = [0.0; COLS];

        let mut i: usize = 0;
        let x: usize = self.x;
        let y: usize = self.y;

        while i < ROWS{
            gridx[i] = (i as f64) * ((SIZE / self.rows) as f64);
            i = i + 1;
        }

        i = 0;

        while i < COLS{
            gridy[i] = (i as f64) * ((SIZE / self.cols) as f64);
            i = i + 1;
        }
        
        let square = rectangle::square(gridx[x], gridy[y], 10.0);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform = c
                .transform
                .trans(x, y)
                .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if (self.x as i32) < self.rows - 1

        {
            self.x += 1;
        } 
        else if (self.y as i32) < self.cols - 1 
        {
            self.y += 1;
            self.x = 0;
        } 
        else 
        {
            self.y = 0;
            self.x = 0;
        }
        println!("x = {0}\ny = {1}\n", self.x, self.y);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rows: 0,
        cols: 0,
        x: 0,
        y: 0
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
