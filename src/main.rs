
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

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let settings = settings::Settings::load();

    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings {
            title: "Rust-2048".to_string(),
            size: settings.window_size,
            fullscreen: false,
            exit_on_esc: true,
            background_color: [
                settings.window_background_color[0],
                settings.window_background_color[1],
                settings.window_background_color[2],
                1.0,
            ],
        }
    );

    let mut asset_store = AssetStore::from_folder(settings.asset_folder.as_slice());

    let mut app = app::App::new(&settings);

    app.run(&mut game_window, &mut asset_store);
}

