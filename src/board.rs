
use std::iter::range_step;
use collections::hashmap::HashSet;
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

        loop {
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

        if self.is_locking() {
            return;
        }
        let mut tiles_need_removed = HashSet::<uint>::new();
        let mut tiles_need_added = Vec::<Tile>::new();
        for i in range(0, self.tiles.len()) {
            let tile1 = self.tiles.get(i);
            if tile1.status != TileStatic {
                continue;
            }
            for j in range(i + 1, self.tiles.len()) {
                let tile2 = self.tiles.get(j);
                if tile2.status != TileStatic
                   || tile1.tile_x != tile2.tile_x
                   || tile1.tile_y != tile2.tile_y {
                    continue;
                }

                tiles_need_removed.insert(i);
                tiles_need_removed.insert(j);
                tiles_need_added.push(Tile::new_combined(tile1.score + tile2.score, tile1.tile_x, tile1.tile_y));
                break;
            }
        }

        if tiles_need_removed.len() > 0 {
            let mut tiles = Vec::<Tile>::new();
            for i in range(0, self.tiles.len()) {
                if !tiles_need_removed.contains(&i) {
                    tiles.push(*self.tiles.get(i));
                }
            }
            self.tiles = tiles.append(tiles_need_added.as_slice());
        }
    }

    pub fn render(&self, c: &Context, gl: &mut Gl) {
        self.render_board(c, gl);
        self.render_tiles(c, gl);
    }

    pub fn merge_from_bottom_to_top(&mut self) {
        self.merge_col(0, settings::TILE_HEIGHT, 1);
    }

    pub fn merge_from_top_to_bottom(&mut self) {
        self.merge_col(settings::TILE_HEIGHT - 1, -1, -1);
    }

    fn merge_col(&mut self, y_start: int, y_end: int, y_step: int) {
        if self.is_locking() {
            return;
        }

        let mut need_generate = false;
        loop {
            // move all tiles to right place
            for col in range(0, settings::TILE_WIDTH) {
                for row in range_step(y_start, y_end, y_step) {
                    match self.get_mut_tile(col, row) {
                        None => {
                            match self.get_mut_next_tile(col, row, 0, y_step) {
                                Some(ref mut tile) => {
                                    println!("move ({}, {}) to ({}, {})",
                                             tile.tile_x, tile.tile_y, col, row);
                                    need_generate = true;
                                    tile.start_moving(col, row);
                                },
                                _ => {},
                            }
                        },
                        _ => {},
                    }
                }
            }

            let mut did_merged = false;
            for col in range(0, settings::TILE_WIDTH) {
                let mut found = false;
                let mut sx = 0;
                let mut sy = 0;
                let mut dx = 0;
                let mut dy = 0;
                for row in range_step(y_start, y_end, y_step) {
                    match self.get_tile(col, row) {
                        Some(ref d_tile) => {
                            match self.get_next_tile(col, row, 0, y_step) {
                                Some(ref s_tile)
                                if d_tile.score == s_tile.score
                                   && self.get_tile_count(d_tile.tile_x, d_tile.tile_y) == 1 => {
                                    found = true;
                                    dx = d_tile.tile_x;
                                    dy = d_tile.tile_y;
                                    sx = s_tile.tile_x;
                                    sy = s_tile.tile_y;
                                    break;
                                },
                                _ => {},
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                if found {
                    need_generate = true;
                    did_merged = true;
                    let mut tile = self.get_mut_tile(sx, sy);
                    let tile = tile.get_mut_ref();
                    tile.start_moving(dx, dy);
                    println!("merge ({}, {}) to ({}, {})", sx, sy, dx, dy);
                }
            }

            if !did_merged {
                break;
            }
        }
        if need_generate {
            self.generate_tile();
        }
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

        let mut need_generate = false;
        loop {
            // move all tiles to right place
            for row in range(0, settings::TILE_HEIGHT) {
                for col in range_step(x_start, x_end, x_step) {
                    match self.get_mut_tile(col, row) {
                        None => {
                            match self.get_mut_next_tile(col, row, x_step, 0) {
                                Some(ref mut tile) => {
                                    println!("move ({}, {}) to ({}, {})", tile.tile_x, tile.tile_y, col, row);
                                    need_generate = true;
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
            let mut did_merged = false;
            for row in range(0, settings::TILE_HEIGHT) {
                let mut found = false;
                let mut sx = 0;
                let mut sy = 0;
                let mut dx = 0;
                let mut dy = 0;
                for col in range_step(x_start, x_end, x_step) {
                    match self.get_tile(col, row) {
                        Some(ref d_tile) => {
                            match self.get_next_tile(col, row, x_step, 0) {
                                Some(ref s_tile)
                                if d_tile.score == s_tile.score
                                   && self.get_tile_count(d_tile.tile_x, d_tile.tile_y) == 1 => {
                                    found = true;
                                    dx = d_tile.tile_x;
                                    dy = d_tile.tile_y;
                                    sx = s_tile.tile_x;
                                    sy = s_tile.tile_y;
                                    break;
                                },
                                _ => {},
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                if found {
                    need_generate = true;
                    did_merged = true;
                    let mut tile = self.get_mut_tile(sx, sy);
                    let tile = tile.get_mut_ref();
                    tile.start_moving(dx, dy);
                    println!("merge ({}, {}) to ({}, {})", sx, sy, dx, dy);
                }
            }

            if !did_merged {
                break;
            }
        }

        if need_generate {
            self.generate_tile();
        }
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

    fn get_tile_count(&self, x: int, y: int) -> int {
        let mut count = 0;
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                count += 1;
            }
        }
        count
    }

    fn render_board(&self, c: &Context, gl: &mut Gl) {
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
        for _ in range(0, settings::TILE_HEIGHT) {
            for _ in range(0, settings::TILE_WIDTH) {
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
        for tile in self.tiles.iter() {
            tile.render(c, gl);
        }
    }
}

