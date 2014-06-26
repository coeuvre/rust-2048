
use std::iter::range_step;
use std::collections::hashmap::HashSet;
use std::rand::random;
use graphics::*;
use piston::*;
use opengl_graphics::{
    Gl,
};
use number_renderer::NumberRenderer;
use settings::Settings;
use tile::{
    Tile,
    TileStatic,
};

pub struct Board<'a> {
    tiles: Vec<Tile<'a>>,
    score: int,

    settings: &'a Settings,
}

impl<'a> Board<'a> {
    pub fn new(settings: &'a Settings) -> Board<'a> {
        let mut board = Board {
            tiles: Vec::<Tile>::new(),
            score: 0,
            settings: settings,
        };
        board.generate_tile();
        board.generate_tile();
        board
    }

    pub fn generate_tile(&mut self) {
        if self.tiles.len() == (self.settings.tile_width * self.settings.tile_height) as uint {
            return;
        }

        loop {
            let x = (random::<uint>() % self.settings.tile_width as uint) as int;
            let y = (random::<uint>() % self.settings.tile_height as uint) as int;
            if self.get_tile(x, y).is_none() {
                self.tiles.push(Tile::new(self.settings, 2, x, y));
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
        let mut score_to_added = 0;
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
                tiles_need_added.push(Tile::new_combined(self.settings, tile1.score + tile2.score, tile1.tile_x, tile1.tile_y));
                score_to_added += tile1.score + tile2.score;
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
            self.add_score(score_to_added);
        }
    }

    pub fn render(&mut self, number_renderer: &NumberRenderer, c: &Context, gl: &mut Gl) {
        number_renderer.render(
            self.score as u32,
            self.settings.best_rect[0] + self.settings.best_rect[2] / 2.0,
            self.settings.best_rect[1] + self.settings.best_rect[3] / 2.0,
            self.settings.best_rect[2],
            self.settings.text_light_color, c, gl);

        self.render_board(c, gl);
        self.render_tiles(number_renderer, c, gl);
    }

    pub fn merge_from_bottom_to_top(&mut self) {
        let height = self.settings.tile_height;
        self.merge_col(0, height, 1);
    }

    pub fn merge_from_top_to_bottom(&mut self) {
        let height = self.settings.tile_height;
        self.merge_col(height - 1, -1, -1);
    }

    fn merge_col(&mut self, y_start: int, y_end: int, y_step: int) {
        if self.is_locking() {
            return;
        }

        let mut need_generate = false;
        loop {
            // move all tiles to right place
            for col in range(0, self.settings.tile_width) {
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
            for col in range(0, self.settings.tile_width) {
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
        let width = self.settings.tile_width;
        self.merge_row(width - 1, -1, -1);
    }

    pub fn merge_from_right_to_left(&mut self) {
        let width = self.settings.tile_width;
        self.merge_row(0, width, 1);
    }

    fn merge_row(&mut self, x_start: int, x_end: int, x_step: int) {
        if self.is_locking() {
            return;
        }

        let mut need_generate = false;
        loop {
            // move all tiles to right place
            for row in range(0, self.settings.tile_height) {
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
            for row in range(0, self.settings.tile_height) {
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
    fn get_next_tile<'b>(&'b self, x: int, y: int, step_x: int, step_y: int) -> Option<&'b Tile<'a>> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        while x >= 0 && x < self.settings.tile_width
              && y >= 0 && y < self.settings.tile_height {
            let tile = self.get_tile(x, y);
            if tile.is_some() {
                return tile;
            }
            x += step_x;
            y += step_y;
        }
        None
    }

    fn get_mut_next_tile<'b>(&'b mut self, x: int, y: int, step_x: int, step_y: int) -> Option<&'b mut Tile<'a>> {
        let mut x = x + step_x;
        let mut y = y + step_y;
        let mut found = false;
        while x >= 0 && x < self.settings.tile_width
              && y >= 0 && y < self.settings.tile_height {
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

    fn get_tile<'b>(&'b self, x: int, y: int) -> Option<&'b Tile<'a>> {
        for tile in self.tiles.iter() {
            if tile.tile_x == x && tile.tile_y == y {
                return Some(tile);
            }
        }
        None
    }

    fn get_mut_tile<'b>(&'b mut self, x: int, y: int) -> Option<&'b mut Tile<'a>> {
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
         .rect(self.settings.board_padding,
               self.settings.board_padding + self.settings.board_offset_y,
               self.settings.board_size[0],
               self.settings.board_size[1])
         .rgba(self.settings.tile_background_color[0],
               self.settings.tile_background_color[1],
               self.settings.tile_background_color[2],
               1.0)
         .draw(gl);

        let mut x = self.settings.board_padding + self.settings.tile_padding;
        let mut y = self.settings.board_padding + self.settings.board_offset_y + self.settings.tile_padding;
        for _ in range(0, self.settings.tile_height) {
            for _ in range(0, self.settings.tile_width) {
                c.view()
                 .rect(x, y, self.settings.tile_size, self.settings.tile_size)
                 .rgba(self.settings.tiles_colors.get(0)[0],
                       self.settings.tiles_colors.get(0)[1],
                       self.settings.tiles_colors.get(0)[2],
                       1.0)
                 .draw(gl);

                x += self.settings.tile_padding + self.settings.tile_size;
            }
            x = self.settings.board_padding + self.settings.tile_padding;
            y += self.settings.tile_padding + self.settings.tile_size;
        }
    }

    fn render_tiles(&self, number_renderer: &NumberRenderer, c: &Context, gl: &mut Gl) {
        for tile in self.tiles.iter() {
            tile.render(number_renderer, c, gl);
        }
    }

    fn add_score(&mut self, score: int) {
        self.score += score;
        println!("Score: {}", self.score);
    }
}

