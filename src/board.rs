
use graphics::*;
use piston::*;
use settings;
use tile::Tile;

pub struct Board {
    center: [f64, ..2],
    tiles: [[Option<Tile>, ..settings::TILE_WIDTH], ..settings::TILE_HEIGHT],
    pub test_tile: Tile,
}

impl Board {
    pub fn new() -> Board {
        Board {
            center: [0.0, 0.0],
            tiles: [
                [None, None, None, None],
                [None, None, None, None],
                [None, None, None, None],
                [None, None, None, None],
            ],
            test_tile: Tile::new(16, 0, 0),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.test_tile.update(dt);
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        self.render_board(c, gl);
        self.render_tiles(c, gl);
        self.test_tile.render(c, gl);
    }

    fn render_board(&self, c: &Context, gl: &mut Gl) {
        let width = settings::TILE_SIZE * settings::TILE_WIDTH as f64 + settings::TILE_PADDING * (settings::TILE_WIDTH + 1) as f64;
        let height = settings::TILE_SIZE * settings::TILE_HEIGHT as f64 + settings::TILE_PADDING * (settings::TILE_HEIGHT + 1) as f64;
        c.view()
         .rect(settings::BOARD_PADDING,
               settings::BOARD_PADDING + settings::BOARD_OFFSET_Y,
               settings::BOARD_SIZE[0],
               settings::BOARD_SIZE[1])
         .rgba(settings::TILE_BACKGROUND_COLOR[0],
               settings::TILE_BACKGROUND_COLOR[1],
               settings::TILE_BACKGROUND_COLOR[2],
               settings::TILE_BACKGROUND_COLOR[3])
         .fill(gl);

        let mut x = settings::BOARD_PADDING + settings::TILE_PADDING;
        let mut y = settings::BOARD_PADDING + settings::BOARD_OFFSET_Y + settings::TILE_PADDING;
        for row in range(0, settings::TILE_HEIGHT) {
            for col in range(0, settings::TILE_WIDTH) {
                c.view()
                 .rect(x, y, settings::TILE_SIZE, settings::TILE_SIZE)
                 .rgba(settings::TILES_COLOR[0][0],
                       settings::TILES_COLOR[0][1],
                       settings::TILES_COLOR[0][2],
                       settings::TILES_COLOR[0][3])
                 .fill(gl);

                x += settings::TILE_PADDING + settings::TILE_SIZE;
            }
            x = settings::BOARD_PADDING + settings::TILE_PADDING;
            y += settings::TILE_PADDING + settings::TILE_SIZE;
        }
    }

    fn render_tiles(&self, c: &Context, gl: &mut Gl) {
        for row in range(0, settings::TILE_HEIGHT) {
            for col in range(0, settings::TILE_WIDTH) {
                match self.tiles[row][col] {
                    Some(ref tile) => {
                        tile.render(c, gl);
                    },
                    _ => {},
                }
            }
        }
    }
}

