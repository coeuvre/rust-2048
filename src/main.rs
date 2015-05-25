extern crate rustc_serialize;
extern crate rand;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::event::*;
use piston::window::{ WindowSettings, Size };
use sdl2_window::Sdl2Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod board;
mod number_renderer;
mod settings;
mod tile;

fn main() {
    let settings = settings::Settings::load();

    let window = Sdl2Window::new(
        OpenGL::_3_2,
        WindowSettings::new(
            "Rust-2048".to_string(),
            Size { width: settings.window_size[0], height: settings.window_size[1] })
            .exit_on_esc(true)
    );

    let mut app = app::App::new(&settings);

    app.load();

    let mut gl = GlGraphics::new(OpenGL::_3_2);

    for e in window.events() {
        use piston::event::{ RenderEvent, PressEvent };

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
