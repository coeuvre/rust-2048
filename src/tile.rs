
use graphics::*;
use piston::*;
use settings;

#[deriving(Clone, Eq)]
pub enum TileState {
    TileStatic,
    /// (t, x, y)
    TileMoving(f64, f64, f64),
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
            status: TileStatic,
        }
    }

    fn tile_to_pos(tile_x: int, tile_y: int) -> (f64, f64) {
        let x = settings::BOARD_PADDING + tile_x as f64 * settings::TILE_SIZE + (tile_x + 1) as f64 * settings::TILE_PADDING;
        let y = settings::BOARD_PADDING + settings::BOARD_OFFSET_Y + tile_y as f64 * settings::TILE_SIZE + (tile_y + 1) as f64 * settings::TILE_PADDING;
        (x, y)
    }

    pub fn start_moving(&mut self, destination_tile_x: int, destination_tile_y: int) {
        let (x, y) = Tile::tile_to_pos(self.tile_x, self.tile_y);
        self.status = TileMoving(settings::TILE_MOVE_TIME, x, y);
        self.tile_x = destination_tile_x;
        self.tile_y = destination_tile_y;
    }

    pub fn update(&mut self, dt: f64) {
        match self.status {
            TileMoving(t, x, y) => {
                if dt >= t {
                    self.status = TileStatic;
                } else {
                    let (dx, dy) = Tile::tile_to_pos(self.tile_x, self.tile_y);
                    let factor = dt / t;
                    self.status = TileMoving(t - dt, x + factor * (dx - x), y + factor * (dy - y));
                }
            },
            TileStatic => {},
        }
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        let mut pos = (0.0, 0.0);
        match self.status {
            TileMoving(_, x, y) => {
                pos = (x, y);
            },
            TileStatic => {
                pos = Tile::tile_to_pos(self.tile_x, self.tile_y);
            },
        }
        let (x, y) = pos;
        let color = self.get_color();
        c.view()
         .rect(x, y, settings::TILE_SIZE, settings::TILE_SIZE)
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

