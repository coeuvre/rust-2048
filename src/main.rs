
#![feature(globs)]

extern crate serialize;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;
extern crate sdl2_game_window;

use piston::*;
use sdl2_game_window::GameWindowSDL2;

mod app;
mod board;
mod number_renderer;
mod settings;
mod tile;

fn main() {
    let settings = settings::Settings::load();

    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-2048".to_string(),
            size: settings.window_size,
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = app::App::new(&settings);

    app.load();

    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
    };

    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(ref args) => {
                app.render(args);
            },
            Update(ref args) => {
                app.update(args);
            },
            KeyPress(ref args) => {
                app.key_press(args);
            },
            _ => {},
        }
    }
}

