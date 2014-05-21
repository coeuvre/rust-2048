
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

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        self.board.update(dt);
    }

    fn load(&mut self, _asset_store: &mut AssetStore) {
        self.board.generate_tile();
        self.board.generate_tile();
    }

    fn key_press(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        if key == keyboard::Left {
            self.board.merge_from_right_to_left();
        }
        if key == keyboard::Right {
            self.board.merge_from_left_to_right();
        }
    }

    fn key_release(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}
}

