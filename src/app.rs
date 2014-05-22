
use graphics::*;
use piston::*;

use board::Board;
use number_renderer::NumberRenderer;

pub struct App {
    board: Board,
    number_renderer: Option<NumberRenderer>,
}

impl App {
    pub fn new() -> App {
        App {
            board: Board::new(),
            number_renderer: None,
        }
    }
}

impl Game for App {
    fn load(&mut self, asset_store: &mut AssetStore) {
        self.number_renderer = Some(NumberRenderer::new(asset_store));
    }

    fn render(&self, c: &Context, gl: &mut Gl) {
        self.board.render(self.number_renderer.get_ref(), c, gl);
    }

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        self.board.update(dt);
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
        if key == keyboard::Up {
            self.board.merge_from_bottom_to_top();
        }
        if key == keyboard::Down {
            self.board.merge_from_top_to_bottom();
        }
        if key == keyboard::Space {
            self.board = Board::new();
        }
    }

    fn key_release(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}
}

