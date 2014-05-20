
use graphics::*;
use piston::*;
use settings;

enum TileState {
    TileStatic,
    /// (x, y)
    TileMoving(f64, f64),
}

pub struct Tile {
    score: int,
    tile_x: int,
    tile_y: int,
    status: TileState,
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

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        let x = settings::BOARD_PADDING + self.tile_x as f64 * settings::TILE_SIZE + (self.tile_x + 1) as f64 * settings::TILE_PADDING;
        let y = settings::BOARD_PADDING + settings::BOARD_OFFSET_Y + self.tile_y as f64 * settings::TILE_SIZE + (self.tile_y + 1) as f64 * settings::TILE_PADDING;

        let color = self.get_color();
        c.view().rect(x, y, settings::TILE_SIZE, settings::TILE_SIZE).rgba(color[0], color[1], color[2], color[3]).fill(gl);
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
