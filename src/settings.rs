
pub static ASSET_FOLDER: &'static str = "assets";

pub static WINDOW_SIZE: [u32, ..2] = [
    (BOARD_PADDING * 2.0 + BOARD_SIZE[0]) as u32,
    (BOARD_PADDING * 2.0 + BOARD_SIZE[1] + BOARD_OFFSET_Y) as u32,
];
pub static WINDOW_BACKGROUND_COLOR: [f32, ..4] = [250.0 / 255.0, 248.0 / 255.0, 239.0 / 255.0, 1.0];

pub static BOARD_PADDING: f64 = 12.0;
pub static BOARD_SIZE: [f64, ..2] = [
    TILE_SIZE * TILE_WIDTH as f64 + TILE_PADDING * (TILE_WIDTH + 1) as f64,
    TILE_SIZE * TILE_HEIGHT as f64 + TILE_PADDING * (TILE_HEIGHT + 1) as f64,
];
pub static BOARD_OFFSET_Y: f64 = 128.0;

pub static TILE_WIDTH: uint = 4;
pub static TILE_HEIGHT: uint = 4;
pub static TILE_SIZE: f64 = 96.0;
pub static TILE_PADDING: f64 = 16.0;
pub static TILE_BACKGROUND_COLOR: [f32, ..4] = [187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0, 1.0];
pub static TILES_COLOR: [[f32, ..4], ..10] = [
    // empty color
    [204.0 / 255.0, 192.0 / 255.0, 179.0 / 255.0, 1.0],
    // 2 color
    [238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0, 1.0],
    // 4 color
    [237.0 / 255.0, 224.0 / 255.0, 200.0 / 255.0, 1.0],
    // 8 color
    [242.0 / 255.0, 177.0 / 255.0, 121.0 / 255.0, 1.0],
    // 16 color
    [245.0 / 255.0, 149.0 / 255.0, 99.0 / 255.0, 1.0],
    // 32 color
    [246.0 / 255.0, 124.0 / 255.0, 95.0 / 255.0, 1.0],
    // 64 color
    [246.0 / 255.0, 94.0 / 255.0, 59.0 / 255.0, 1.0],
    // 128 color
    [237.0 / 255.0, 207.0 / 255.0, 114.0 / 255.0, 1.0],
    // 256 color
    [237.0 / 255.0, 204.0 / 255.0, 97.0 / 255.0, 1.0],
    // 512 color
    [237.0 / 255.0, 200.0 / 255.0, 80.0 / 255.0, 1.0],
];
pub static TILE_UNKNOW_COLOR: [f32, ..4] = [0.8, 0.0, 0.0, 1.0];
pub static TILE_MOVE_TIME: f64 = 0.08;

pub static LABEL_COLOR: [f32, ..4] = [187.0 / 255.0, 173.0 / 255.0, 160.0 / 255.0, 1.0];
pub static BUTTON_COLOR: [f32, ..4] = [142.0 / 255.0, 122.0 / 255.0, 102.0 / 255.0, 1.0];
