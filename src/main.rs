#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate piston_window;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston_window::*;
use sdl2_window::Sdl2Window;

mod app;
mod board;
mod number_renderer;
mod settings;
mod tile;

fn main() {
	use opengl_graphics::GlGraphics;	
    let settings = settings::Settings::load();

	let (width, height) = (settings.window_size[0], 
	                       settings.window_size[1]);

    // according to piston WindowSettings documentation, OpenGL::V3_2 is the default version
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Rust-2048", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new(&settings);

    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(ref args) = e.update_args() {
           // TODO: only update if necessary
           // println!("update");
           app.update(args);
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }
}
