
use std::iter::range_step;
use rand::random;
use graphics::*;
use piston::*;
use settings;
use tile::{
    Tile,
    TileStatic,
};

pub struct Board {
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: Vec::<Tile>::new(),
        }
    }

    pub fn generate_tile(&mut self) {
        if self.tiles.len() == (settings::TILE_WIDTH * settings::TILE_HEIGHT) as uint {
            return;
        }

        'generating: loop {
            let x = (random::<uint>() % settings::TILE_WIDTH as uint) as int;
            let y = (random::<uint>() % settings::TILE_HEIGHT as uint) as int;
            if self.get_tile(x, y).is_none() {
                self.tiles.push(Tile::new(2, x, y));
                break;
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for tile in self.tiles.mut_iter() {
            tile.update(dt);
        }
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        self.render_board(c, gl);
        self.render_tiles(c, gl);
    }

    pub fn can_merge(&self) -> bool {
        for row in range(0, settings::TILE_HEIGHT) {
            if !self.can_merge_row(row) {
                return false;
            }
        }
        true
    }

    pub fn merge_from_left_to_right(&mut self) {
        self.merge_row(settings::TILE_WIDTH - 1, -1, -1);
    }

    pub fn merge_from_right_to_left(&mut self) {
        self.merge_row(0, settings::TILE_WIDTH, 1);
    }

    fn merge_row(&mut self, x_start: int, x_end: int, x_step: int) {
        if self.is_locking() {
            return;
        }

        // move all tiles to right place
        for row in range(0, settings::TILE_HEIGHT) {
            for col in range_step(x_start, x_end, x_step) {
                match self.get_mut_tile(col, row) {
                    None => {
                        match self.get_mut_next_tile(col, row, x_step, 0) {
                            Some(ref mut tile) => {
                                tile.start_moving(col, row);
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }
        }

        // merge

        self.generate_tile();
    }

    fn can_merge_row(&self, row: int) -> bool {
        for col in range(0, settings::TILE_WIDTH) {
            match self.get_tile(col, row) {
                Some(ref tile) => {
                    match self.get_next_tile(tile.tile_x, tile.tile_y, 1, 0) {
                        Some(ref s_tile) => {
                            if tile.score == s_tile.score {
                                return true;
                            }
                        },
                        _ => {},
                    }
                },
                None => {},
            }
        }
        false
    }

    fn is_locking(&self) -> bool {
        for tile in self.tiles.iter() {
            if tile.status != TileStatic {
                return true;
            }
        }
        false
    }

    /// Returns next tile right besides (x, y)
    fn get_next_tile<'a>(&'a self, x: int, y: int, step_x: int, step_y: int) -> Option<&'a Tile> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        while x >= 0 && x < settings::TILE_WIDTH
              && y >= 0 && y < settings::TILE_HEIGHT {
            let tile = self.get_tile(x, y);
            if tile.is_some() {
                return tile;
            }
            x += step_x;
            y += step_y;
        }
        None
    }

    fn get_mut_next_tile<'a>(&'a mut self, x: int, y: int, step_x: int, step_y: int) -> Option<&'a mut Tile> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        let mut found = false;
        while x >= 0 && x < settings::TILE_WIDTH
              && y >= 0 && y < settings::TILE_HEIGHT {
            let tile = self.get_tile(x, y);
            if tile.is_some() {
                found = true;
                break;
            }
            x += step_x;
            y += step_y;
        }

        if found {
            self.get_mut_tile(x, y)
        } else {
            None
        }
    }

    fn get_tile<'a>(&'a self, x: int, y: int) -> Option<&'a Tile> {
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }
        None
    }

    fn get_mut_tile<'a>(&'a mut self, x: int, y: int) -> Option<&'a mut Tile> {
        for tile in self.tiles.mut_iter() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }
        None
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
                match self.get_tile(col, row) {
                    Some(ref tile) => {
                        tile.render(c, gl);
                    },
                    _ => {},
                }
            }
        }
    }
}

