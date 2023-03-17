use std::path::{ Path, PathBuf };

use piston_window::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use board::Board;
use number_renderer::NumberRenderer;
use settings::Settings;

pub struct App<'a> {
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
    settings: &'a Settings,

    logo: Option<GlTexture>,
    comment1: Option<GlTexture>,
    comment2: Option<GlTexture>,
    window_background_color: [f32; 4],
}

fn rgb2rgba(c: [f32; 3]) -> [f32; 4] { [c[0], c[1], c[2], 1.0] }

impl<'a> App<'a> {
    pub fn new(settings: &'a Settings) -> App<'a> {
        App {
            board: Board::new(settings),
            number_renderer: None,
            settings: settings,

            logo: None,
            comment1: None,
            comment2: None,
            window_background_color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    fn render_ui(&self, c: &Context, gl: &mut GlGraphics) {		
        Image::new_color(rgb2rgba(self.settings.text_dark_color))
            .draw(self.logo.iter().next().unwrap(),
                  &DrawState::default(),
                  c.trans(self.settings.board_padding,self.settings.board_padding).transform,
                  gl);

        Rectangle::new(rgb2rgba(self.settings.label_color))
            .draw(self.settings.best_rect,
                  &DrawState::default(),
                  c.transform,
                  gl);

        let comment1_offset_y = self.settings.comment1_offset_y;
        let comment1 = self.comment1.as_ref().unwrap();
        App::render_comment(self.settings, comment1, comment1_offset_y, c, gl);
        let comment2_offset_y = self.settings.comment2_offset_y;
        let comment2 = self.comment2.as_ref().unwrap();
        App::render_comment(self.settings, comment2, comment2_offset_y, c, gl);
    }

    fn render_comment(settings: &Settings, comment: &GlTexture, y: f64, c: &Context, gl: &mut GlGraphics) {
        let (width, height) = comment.get_size();
        let w = settings.window_size[0] as f64 - 2.0 * settings.board_padding;
        let h = height as f64 * w / width as f64;

        Image::new_color(rgb2rgba(settings.text_dark_color))
            .rect([settings.board_padding, y, w, h])
            .draw( comment,
                   &DrawState::default(),
                   c.transform,
                   gl);
    }

    pub fn load(&mut self) {
        let mut asset_root = PathBuf::new();
        asset_root.push(Path::new(&self.settings.asset_folder));

        let mut logo_path = asset_root.clone();
        logo_path.push(Path::new("logo.png"));
        let mut comment1_path = asset_root.clone();
        comment1_path.push(Path::new("comment1.png"));
        let mut comment2_path = asset_root.clone();
        comment2_path.push(Path::new("comment2.png"));

        self.number_renderer = Some(NumberRenderer::new());
        let texture_settings = TextureSettings::new();
        self.logo = Some(GlTexture::from_path(&logo_path, &texture_settings).unwrap());
        self.comment1 = Some(GlTexture::from_path(&comment1_path, &texture_settings).unwrap());
        self.comment2 = Some(GlTexture::from_path(&comment2_path, &texture_settings).unwrap());
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let area = args.window_size;
        let ref c = Context::new_abs(area[0], area[1]);

        let w_bg_col = self.window_background_color;
        let ref nr = self.number_renderer;

        gl.draw(args.viewport(), |_, gl| {
            clear(w_bg_col, gl);
            self.render_ui(c, gl);
            self.board.render(nr.iter().next().unwrap(), c, gl);
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.board.update(args.dt);
    }

    pub fn key_press(&mut self, args: &Button) {
		use piston_window::Button::Keyboard;
		
        if *args == Keyboard(Key::Left) {
            self.board.merge_from_right_to_left();
        }

        if *args == Keyboard(Key::Right) {
            self.board.merge_from_left_to_right();
        }

        if *args == Keyboard(Key::Up) {
            self.board.merge_from_bottom_to_top();
        }

        if *args == Keyboard(Key::Down) {
            self.board.merge_from_top_to_bottom();
        }

        if *args == Keyboard(Key::Space) {
            self.board = Board::new(self.settings);
        }
    }
}
