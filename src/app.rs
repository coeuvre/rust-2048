
use graphics::*;
use piston::*;

use board::Board;
use number_renderer::NumberRenderer;
use settings::Settings;

pub struct App<'a> {
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
    settings: &'a Settings,

    logo: Option<Texture>,
    comment1: Option<Texture>,
    comment2: Option<Texture>,
}

impl<'a> App<'a> {
    pub fn new(settings: &'a Settings) -> App<'a> {
        App {
            board: Board::new(settings),
            number_renderer: None,
            settings: settings,

            logo: None,
            comment1: None,
            comment2: None,
        }
    }
}

impl<'a> App<'a> {
    fn render_ui(&self, c: &Context, gl: &mut Gl) {
        // logo
        c.trans(self.settings.board_padding, self.settings.board_padding)
         .image(self.logo.get_ref())
         .rgb(self.settings.text_dark_color[0],
              self.settings.text_dark_color[1],
              self.settings.text_dark_color[2])
         .draw(gl);

        c.view()
         .rect(self.settings.best_rect[0],
               self.settings.best_rect[1],
               self.settings.best_rect[2],
               self.settings.best_rect[3])
         .rgba(self.settings.label_color[0],
               self.settings.label_color[1],
               self.settings.label_color[2],
               1.0)
         .fill(gl);

        self.render_comment(self.comment1.get_ref(), self.settings.comment1_offset_y, c, gl);
        self.render_comment(self.comment2.get_ref(), self.settings.comment2_offset_y, c, gl);
    }

    fn render_comment(&self, comment: &Texture, y: f64, c: &Context, gl: &mut Gl) {
        let (width, height) = comment.get_size();
        let w = self.settings.window_size[0] as f64 - 2.0 * self.settings.board_padding;
        let h = height as f64 * w / width as f64;
        c.rect(self.settings.board_padding, y, w, h)
         .image(comment)
         .rgb(self.settings.text_dark_color[0],
              self.settings.text_dark_color[1],
              self.settings.text_dark_color[2])
         .draw(gl);
    }
}

impl<'a> Game for App<'a> {
    fn load(&mut self, asset_store: &mut AssetStore) {
        self.number_renderer = Some(NumberRenderer::new(asset_store));

        self.logo = Some(Texture::from_path(&asset_store.path("logo.png").unwrap()).unwrap());
        self.comment1 = Some(Texture::from_path(&asset_store.path("comment1.png").unwrap()).unwrap());
        self.comment2 = Some(Texture::from_path(&asset_store.path("comment2.png").unwrap()).unwrap());
    }

    fn render(&self, _ext_dt: f64, c: &Context, gl: &mut Gl) {
        self.render_ui(c, gl);
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
            self.board = Board::new(self.settings);
        }
    }

    fn key_release(
        &mut self,
        _key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {}
}

