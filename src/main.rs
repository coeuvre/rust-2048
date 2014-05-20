
#![feature(globs)]

extern crate rand;

extern crate graphics;
extern crate piston;

use piston::*;

mod app;
mod board;
mod settings;

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings::new (
            "Rust-2048".to_owned(),
            settings::WINDOW_SIZE,
            false,
            true,
            settings::WINDOW_BACKGROUND_COLOR,
        )
    );

    let mut asset_store = AssetStore::from_folder(settings::ASSET_FOLDER);

    let mut app = app::App::new();

    app.run(&mut game_window, &mut asset_store);
}

