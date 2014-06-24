
#![feature(globs)]

extern crate serialize;

extern crate graphics;
extern crate piston;

use piston::*;

mod app;
mod board;
mod number_renderer;
mod settings;
mod tile;

fn main() {
    let settings = settings::Settings::load();

    let mut game_window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Rust-2048".to_string(),
            size: settings.window_size,
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = app::App::new(&settings);
    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    app.run(&mut game_window, &game_iter_settings);
}

