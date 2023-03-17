use piston_window::*;
use opengl_graphics::GlGraphics;
use number_renderer::NumberRenderer;
use settings::Settings;

#[derive(Clone, PartialEq)]
pub enum TileState {
    TileStatic,
    /// (t, x, y, origin_x, origin_y)
    TileMoving(f64, f64, f64, i32, i32),
    /// (t, size)
    TileNew(f64, f64),
    /// (t, size)
    TileCombine(f64, f64),
}

#[derive(Clone)]
pub struct Tile<'a> {
    pub score: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub status: TileState,

    settings: &'a Settings,
}

impl<'a> Tile<'a> {
    pub fn new(settings: &'a Settings, score: i32, tile_x: i32, tile_y: i32) -> Tile<'a> {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileState::TileNew(settings.tile_new_time, 0.0),

            settings: settings,
        }
    }

    pub fn new_combined(settings: &'a Settings, score: i32, tile_x: i32, tile_y: i32) -> Tile<'a> {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileState::TileCombine(settings.tile_combine_time, 1.2 * settings.tile_size),

            settings: settings,
        }
    }

    fn tile_to_pos(&self, tile_x: i32, tile_y: i32) -> (f64, f64) {
        let x = self.settings.board_padding + tile_x as f64 * self.settings.tile_size + (tile_x + 1) as f64 * self.settings.tile_padding;
        let y = self.settings.board_padding + self.settings.board_offset_y + tile_y as f64 * self.settings.tile_size + (tile_y + 1) as f64 * self.settings.tile_padding;
        (x, y)
    }

    pub fn start_moving(&mut self, destination_tile_x: i32, destination_tile_y: i32) {
        match self.status {
            TileState::TileMoving(_, _, _, ox, oy) => {
                let (x, y) = self.tile_to_pos(ox, oy);
                self.status = TileState::TileMoving(self.settings.tile_move_time, x, y, ox, oy);
                self.tile_x = destination_tile_x;
                self.tile_y = destination_tile_y;
            },
            TileState::TileStatic => {
                let (x, y) = self.tile_to_pos(self.tile_x, self.tile_y);
                self.status = TileState::TileMoving(self.settings.tile_move_time, x, y, self.tile_x, self.tile_y);
                self.tile_x = destination_tile_x;
                self.tile_y = destination_tile_y;
            },
            _ => {},
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.status {
            TileState::TileMoving(t, x, y, ox, oy) => {
                if dt >= t {
                    self.status = TileState::TileStatic;
                } else {
                    let (dx, dy) = self.tile_to_pos(self.tile_x, self.tile_y);
                    let factor = dt / t;
                    self.status = TileState::TileMoving(t - dt, x + factor * (dx - x), y + factor * (dy - y), ox, oy);
                }
            },
            TileState::TileNew(t, size) => {
                if dt >= t {
                    self.status = TileState::TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileState::TileNew(t - dt, size + factor * (self.settings.tile_size - size));
                }
            },
            TileState::TileCombine(t, size) => {
                if dt >= t {
                    self.status = TileState::TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileState::TileCombine(t - dt, size + factor * (self.settings.tile_size - size));
                }
            },
            _ => {},
        }
    }

    pub fn render(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut GlGraphics) {
        let mut pos = self.tile_to_pos(self.tile_x, self.tile_y);
        let mut size = (self.settings.tile_size, self.settings.tile_size);

        match self.status {
            TileState::TileMoving(_, x, y, _, _) => {
                pos = (x, y);
            },
            TileState::TileNew(_, s) => {
                size = (s, s);
            },
            TileState::TileCombine(_, s) => {
                size = (s, s);
            },
            _ => {},
        }

        let (x, y) = pos;
        let (w, h) = size;
        let color = self.get_color();

        Rectangle::new([color[0], color[1], color[2], 1.0])
            .draw(rectangle::centered([x + self.settings.tile_size / 2.0,
                                       y + self.settings.tile_size / 2.,
                                       w/2.0, h/2.0]),
                  &DrawState::default(),
                  c.transform,
                  gl);

        let color = if self.score >= 8 {
            self.settings.text_light_color
        } else {
            self.settings.text_dark_color
        };

        number_renderer.render(self.score as u32, x + self.settings.tile_size / 2.0, y + self.settings.tile_size / 2.0, self.settings.tile_size, color, c, gl);
    }

    fn get_color(&self) -> [f32; 3] {
        let i = (self.score as f64).log2() as usize;
        if i > 0 && i < self.settings.tiles_colors.len() {
            self.settings.tiles_colors[i]
        } else {
            self.settings.tile_unknow_color
        }
    }
}
