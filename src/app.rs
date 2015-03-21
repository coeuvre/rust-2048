use graphics::*;
use piston::*;
use piston::events::{ RenderArgs, UpdateArgs };

use board::Board;
use number_renderer::NumberRenderer;
use settings::Settings;
use opengl_graphics::{
    Gl,
    Texture,
};

pub struct App<'a> {
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
    settings: &'a Settings,

    logo: Option<Texture>,
    comment1: Option<Texture>,
    comment2: Option<Texture>,
    window_background_color: [f32; ..4],

    gl: Gl,
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
            window_background_color: [1.0, 1.0, 1.0, 1.0],

            gl: Gl::new(),
        }
    }

    fn render_ui(&mut self, c: &Context) {
        // logo
        c.trans(self.settings.board_padding, self.settings.board_padding)
         .image(self.logo.get_ref())
         .rgb(self.settings.text_dark_color[0],
              self.settings.text_dark_color[1],
              self.settings.text_dark_color[2])
         .draw(&mut self.gl);

        c.view()
         .rect(self.settings.best_rect[0],
               self.settings.best_rect[1],
               self.settings.best_rect[2],
               self.settings.best_rect[3])
         .rgba(self.settings.label_color[0],
               self.settings.label_color[1],
               self.settings.label_color[2],
               1.0)
         .draw(&mut self.gl);

        let comment1_offset_y = self.settings.comment1_offset_y;
        let comment1 = self.comment1.as_ref().unwrap();
        App::render_comment(self.settings, comment1, comment1_offset_y, c, &mut self.gl);
        let comment2_offset_y = self.settings.comment2_offset_y;
        let comment2 = self.comment2.as_ref().unwrap();
        App::render_comment(self.settings, comment2, comment2_offset_y, c, &mut self.gl);
    }

    fn render_comment(settings: &Settings, comment: &Texture, y: f64, c: &Context, gl: &mut Gl) {
        let (width, height) = comment.get_size();
        let w = settings.window_size[0] as f64 - 2.0 * settings.board_padding;
        let h = height as f64 * w / width as f64;
        c.rect(settings.board_padding, y, w, h)
         .image(comment)
         .rgb(settings.text_dark_color[0],
              settings.text_dark_color[1],
              settings.text_dark_color[2])
         .draw(gl);
    }

    pub fn load(&mut self) {
        let asset_store = Path::new(self.settings.asset_folder.as_slice());
        self.number_renderer = Some(NumberRenderer::new(&asset_store));

        self.logo = Some(Texture::from_path(&asset_store.path("logo.png").unwrap()).unwrap());
        self.comment1 = Some(Texture::from_path(&asset_store.path("comment1.png").unwrap()).unwrap());
        self.comment2 = Some(Texture::from_path(&asset_store.path("comment2.png").unwrap()).unwrap());
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.viewport(0, 0, args.width as i32, args.height as i32);
        let ref c = Context::abs(args.width as f64, args.height as f64);
        c.color(self.window_background_color).draw(&mut self.gl);

        self.render_ui(c);
        self.board.render(self.number_renderer.get_ref(), c, &mut self.gl);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.board.update(args.dt);
    }

    pub fn key_press(&mut self, args: &KeyPressArgs) {
        use piston::input::Button::Keyboard;
        use piston::input::keyboard::Key;

        if args.key == Keyboard(Key::Left) {
            self.board.merge_from_right_to_left();
        }
        
        if args.key == Keyboard(Key::Right) {
            self.board.merge_from_left_to_right();
        }
        
        if args.key == Keyboard(Key::Up) {
            self.board.merge_from_bottom_to_top();
        }
        
        if args.key == Keyboard(Key::Down) {
            self.board.merge_from_top_to_bottom();
        }
        
        if args.key == Keyboard(Key::Space) {
            self.board = Board::new(self.settings);
        }
    }
}

