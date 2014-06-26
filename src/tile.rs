
use graphics::*;
use piston::*;
use opengl_graphics::Gl;
use number_renderer::NumberRenderer;
use settings::Settings;

#[deriving(Clone, PartialEq)]
pub enum TileState {
    TileStatic,
    /// (t, x, y, origin_x, origin_x)
    TileMoving(f64, f64, f64, int, int),
    /// (t, size)
    TileNew(f64, f64),
    /// (t, size)
    TileCombine(f64, f64),
}

#[deriving(Clone)]
pub struct Tile<'a> {
    pub score: int,
    pub tile_x: int,
    pub tile_y: int,
    pub status: TileState,

    settings: &'a Settings,
}

impl<'a> Tile<'a> {
    pub fn new(settings: &'a Settings, score: int, tile_x: int, tile_y: int) -> Tile<'a> {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileNew(settings.tile_new_time, 0.0),

            settings: settings,
        }
    }

    pub fn new_combined(settings: &'a Settings, score: int, tile_x: int, tile_y: int) -> Tile<'a> {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileCombine(settings.tile_combine_time, 1.2 * settings.tile_size),

            settings: settings,
        }
    }

    fn tile_to_pos(&self, tile_x: int, tile_y: int) -> (f64, f64) {
        let x = self.settings.board_padding + tile_x as f64 * self.settings.tile_size + (tile_x + 1) as f64 * self.settings.tile_padding;
        let y = self.settings.board_padding + self.settings.board_offset_y + tile_y as f64 * self.settings.tile_size + (tile_y + 1) as f64 * self.settings.tile_padding;
        (x, y)
    }

    pub fn start_moving(&mut self, destination_tile_x: int, destination_tile_y: int) {
        match self.status {
            TileMoving(_, _, _, ox, oy) => {
                let (x, y) = self.tile_to_pos(ox, oy);
                self.status = TileMoving(self.settings.tile_move_time, x, y, ox, oy);
                self.tile_x = destination_tile_x;
                self.tile_y = destination_tile_y;
            },
            TileStatic => {
                let (x, y) = self.tile_to_pos(self.tile_x, self.tile_y);
                self.status = TileMoving(self.settings.tile_move_time, x, y, self.tile_x, self.tile_y);
                self.tile_x = destination_tile_x;
                self.tile_y = destination_tile_y;
            },
            _ => {},
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.status {
            TileMoving(t, x, y, ox, oy) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let (dx, dy) = self.tile_to_pos(self.tile_x, self.tile_y);
                    let factor = dt / t;
                    self.status = TileMoving(t - dt, x + factor * (dx - x), y + factor * (dy - y), ox, oy);
                }
            },
            TileNew(t, size) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileNew(t - dt, size + factor * (self.settings.tile_size - size));
                }
            },
            TileCombine(t, size) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileCombine(t - dt, size + factor * (self.settings.tile_size - size));
                }
            },
            _ => {},
        }
    }

    pub fn render(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut Gl) {
        let mut pos = self.tile_to_pos(self.tile_x, self.tile_y);
        let mut size = (self.settings.tile_size, self.settings.tile_size);
        match self.status {
            TileMoving(_, x, y, _, _) => {
                pos = (x, y);
            },
            TileNew(_, s) => {
                size = (s, s);
            },
            TileCombine(_, s) => {
                size = (s, s);
            },
            _ => {},
        }
        let (x, y) = pos;
        let (w, h) = size;
        let color = self.get_color();
        c.view()
         .rect_centered(x + self.settings.tile_size / 2.0,
                        y + self.settings.tile_size / 2.0,
                        w / 2.0, h / 2.0)
         .rgba(color[0], color[1], color[2], 1.0).draw(gl);

        let color = if self.score >= 8 {
            self.settings.text_light_color
        } else {
            self.settings.text_dark_color
        };
        number_renderer.render(self.score as u32, x + self.settings.tile_size / 2.0, y + self.settings.tile_size / 2.0, self.settings.tile_size, color, c, gl);
    }

    fn get_color(&self) -> [f32, ..3] {
        let i = (self.score as f64).log2() as uint;
        if i > 0 && i < self.settings.tiles_colors.len() {
            *self.settings.tiles_colors.get(i)
        } else {
            self.settings.tile_unknow_color
        }
    }
}

