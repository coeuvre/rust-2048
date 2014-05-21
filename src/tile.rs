
use graphics::*;
use piston::*;
use settings;

#[deriving(Clone, Eq)]
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
pub struct Tile {
    pub score: int,
    pub tile_x: int,
    pub tile_y: int,
    pub status: TileState,
}

impl Tile {
    pub fn new(score: int, tile_x: int, tile_y: int) -> Tile {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileNew(settings::TILE_NEW_TIME, 0.0),
        }
    }

    pub fn new_combined(score: int, tile_x: int, tile_y: int) -> Tile {
        Tile {
            score: score,
            tile_x: tile_x,
            tile_y: tile_y,
            status: TileCombine(settings::TILE_COMBINE_TIME, 1.2 * settings::TILE_SIZE),
        }
    }

    fn tile_to_pos(tile_x: int, tile_y: int) -> (f64, f64) {
        let x = settings::BOARD_PADDING + tile_x as f64 * settings::TILE_SIZE + (tile_x + 1) as f64 * settings::TILE_PADDING;
        let y = settings::BOARD_PADDING + settings::BOARD_OFFSET_Y + tile_y as f64 * settings::TILE_SIZE + (tile_y + 1) as f64 * settings::TILE_PADDING;
        (x, y)
    }

    pub fn start_moving(&mut self, destination_tile_x: int, destination_tile_y: int) {
        match self.status {
            TileMoving(_, _, _, ox, oy) => {
                let (x, y) = Tile::tile_to_pos(ox, oy);
                self.status = TileMoving(settings::TILE_MOVE_TIME, x, y, ox, oy);
                self.tile_x = destination_tile_x;
                self.tile_y = destination_tile_y;
            },
            TileStatic => {
                let (x, y) = Tile::tile_to_pos(self.tile_x, self.tile_y);
                self.status = TileMoving(settings::TILE_MOVE_TIME, x, y, self.tile_x, self.tile_y);
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
                    let (dx, dy) = Tile::tile_to_pos(self.tile_x, self.tile_y);
                    let factor = dt / t;
                    self.status = TileMoving(t - dt, x + factor * (dx - x), y + factor * (dy - y), ox, oy);
                }
            },
            TileNew(t, size) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileNew(t - dt, size + factor * (settings::TILE_SIZE - size));
                }
            },
            TileCombine(t, size) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let factor = dt / t;
                    self.status = TileCombine(t - dt, size + factor * (settings::TILE_SIZE - size));
                }
            }
            _ => {},
        }
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        let mut pos = Tile::tile_to_pos(self.tile_x, self.tile_y);
        let mut size = (settings::TILE_SIZE, settings::TILE_SIZE);
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
         .rect_centered(x + settings::TILE_SIZE / 2.0,
                        y + settings::TILE_SIZE / 2.0,
                        w / 2.0, h / 2.0)
         .rgba(color[0], color[1], color[2], color[3]).fill(gl);
    }

    fn get_color(&self) -> [f32, ..4] {
        let i = (self.score as f64).log2() as uint;
        if i > 0 && i < settings::TILES_COLOR.len() {
            settings::TILES_COLOR[i]
        } else {
            settings::TILE_UNKNOW_COLOR
        }
    }
}

