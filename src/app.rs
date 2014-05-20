
use graphics::*;
use piston::*;

use board::Board;

pub struct App {
    board: Board,
}

impl App {
    pub fn new() -> App {
        App {
            board: Board::new(),
        }
    }
}

impl Game for App {
    fn render(&self, c: &Context, gl: &mut Gl) {
        self.board.render(c, gl);
    }

    fn update(&mut self, _dt: f64, _asset_store: &mut AssetStore) {}

    fn load(&mut self, _asset_store: &mut AssetStore) {}

    fn key_press(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}

    fn key_release(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}
}
