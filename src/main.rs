extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use rand::distributions::{IndependentSample, Range};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};


extern crate som;

use som::som::SOM;
use som::node::Node;


const MAP_DIM: usize  = 80;
const N_ITERS: u32    = 2000 + 1;
const SCALE: u32      = 5;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs, som: &SOM) {

        use graphics::*;

        let square = rectangle::square(0.0, 0.0, SCALE as f64);

        let size = som.get_size();
        let lattice = som.get_lattice();

        self.gl.draw(args.viewport(), |ctr, gl| {
            clear(BLACK, gl);

            for r in 0..size.0 {
                for c in 0..size.1 {
                    let transform = ctr.transform.trans((c as u32*SCALE) as f64,
                                                        (r as u32*SCALE) as f64);
                    let elm = &lattice[r][c];
                    let color = [elm.0, elm.1, elm.2, 1.0];
                    rectangle(color, square, transform, gl);
                }
            }
        });
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Initializing...");
    let mut s = SOM::new((MAP_DIM, MAP_DIM), 0.1, N_ITERS, &mut rng);

    let samples = vec![
        Node(1.0,0.0,0.0),
        Node(0.0,1.0,0.0),
        Node(0.0,0.0,1.0),
        Node(1.0,1.0,0.0),
        Node(1.0,0.0,1.0),
        Node(0.0,1.0,1.0),
        Node(0.5,0.0,0.0),
        Node(0.5,0.5,0.0),
        Node(1.0,0.647,0.0),
    ];

    let samples_range = Range::new(0, samples.len() as u32);

    println!("Running...");

    for _ in 1..N_ITERS {
        let sample_i = samples_range.ind_sample(&mut rng) as usize;
        s.update(&samples[sample_i]);
    }

    println!("Done!");


    let opengl = OpenGL::V3_2;

    // Create a Glutin window
    let mut window: Window = WindowSettings::new(
            "Kohonen Self Organizing Map",
            [MAP_DIM as u32 * SCALE, MAP_DIM as u32 * SCALE]
        )
        .opengl(opengl) .exit_on_esc(true)
        .build() .unwrap();

    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &s);
        }
    }
}
