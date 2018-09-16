extern crate serde;
extern crate serde_json;

use self::serde_json::Error;
use std::env::current_exe;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

static SETTING_FILENAME: &'static str = "settings.json";

pub struct Settings {
    pub asset_folder: String,
    pub window_size: [u32; 2],
    pub window_background_color: [f32; 3],
    pub comment1_offset_y: f64,
    pub comment2_offset_y: f64,
    pub board_padding: f64,
    pub board_size: [f64; 2],
    pub board_offset_y: f64,
    pub tile_width: i32,
    pub tile_height: i32,
    pub tile_size: f64,
    pub tile_padding: f64,
    pub tile_background_color: [f32; 3],
    pub tiles_colors: Vec<[f32; 3]>,
    pub tile_unknow_color: [f32; 3],
    pub tile_move_time: f64,
    pub tile_new_time: f64,
    pub tile_combine_time: f64,
    pub best_rect: [f64; 4],
    pub score_rect: [f64; 4],
    pub label_color: [f32; 3],
    pub button_color: [f32; 3],
    pub text_dark_color: [f32; 3],
    pub text_light_color: [f32; 3],
}

impl Settings {
    pub fn load() -> Settings {
        Settings::from_settings_in_json(&SettingsInJson::load())
    }

    fn from_settings_in_json(s: &SettingsInJson) -> Settings {
        let board_size = [
            s.tile_size * s.tile_width as f64 + s.tile_padding * (s.tile_width + 1) as f64,
            s.tile_size * s.tile_height as f64 + s.tile_padding * (s.tile_height + 1) as f64,
        ];

        let mut tiles_colors = Vec::<[f32; 3]>::new();

        for color in s.tiles_colors.iter() {
            tiles_colors.push([
                color[0] / 255.0,
                color[1] / 255.0,
                color[2] / 255.0,
            ]);
        }

        Settings {
            asset_folder: s.asset_folder.clone(),
            comment1_offset_y: s.comment1_offset_y,
            comment2_offset_y: s.comment2_offset_y,
            window_size: [
                (s.board_padding * 2.0 + board_size[0]) as u32,
                (s.board_padding * 2.0 + board_size[1] + s.board_offset_y) as u32,
            ],
            window_background_color: [
                s.window_background_color[0] / 255.0,
                s.window_background_color[1] / 255.0,
                s.window_background_color[2] / 255.0,
            ],
            board_padding: s.board_padding,
            board_size: board_size,
            board_offset_y: s.board_offset_y,
            tile_width: s.tile_width,
            tile_height: s.tile_height,
            tile_size: s.tile_size,
            tile_padding: s.tile_padding,
            tile_background_color: [
                s.tile_background_color[0] / 255.0,
                s.tile_background_color[1] / 255.0,
                s.tile_background_color[2] / 255.0,
            ],
            tiles_colors: tiles_colors,
            tile_unknow_color: [
                s.tile_unknow_color[0] / 255.0,
                s.tile_unknow_color[1] / 255.0,
                s.tile_unknow_color[2] / 255.0,
            ],
            tile_move_time: s.tile_move_time,
            tile_new_time: s.tile_new_time,
            tile_combine_time: s.tile_combine_time,
            best_rect: [
                s.best_rect[0],
                s.best_rect[1],
                s.best_rect[2],
                s.best_rect[3],
            ],
            score_rect: [
                s.score_rect[0],
                s.score_rect[1],
                s.score_rect[2],
                s.score_rect[3],
            ],
            label_color: [
                s.label_color[0] / 255.0,
                s.label_color[1] / 255.0,
                s.label_color[2] / 255.0,
            ],
            button_color: [
                s.button_color[0] / 255.0,
                s.button_color[1] / 255.0,
                s.button_color[2] / 255.0,
            ],
            text_dark_color: [
                s.text_dark_color[0] / 255.0,
                s.text_dark_color[1] / 255.0,
                s.text_dark_color[2] / 255.0,
            ],
            text_light_color: [
                s.text_light_color[0] / 255.0,
                s.text_light_color[1] / 255.0,
                s.text_light_color[2] / 255.0,
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SettingsInJson {
    asset_folder: String,

    // r g b (0 - 255)
    window_background_color: Vec<f32>,

    comment1_offset_y: f64,
    comment2_offset_y: f64,

    board_padding: f64,
    board_offset_y: f64,

    tile_width: i32,
    tile_height: i32,
    tile_size: f64,
    tile_padding: f64,
    tile_background_color: Vec<f32>,
    tiles_colors: Vec<Vec<f32>>,
    tile_unknow_color: Vec<f32>,

    tile_move_time: f64,
    tile_new_time: f64,
    tile_combine_time: f64,

    best_rect: Vec<f64>,
    score_rect: Vec<f64>,

    label_color: Vec<f32>,
    button_color: Vec<f32>,
    text_dark_color: Vec<f32>,
    text_light_color: Vec<f32>,
}

impl SettingsInJson {
    pub fn default_settings() -> SettingsInJson {
        let mut tiles_colors = Vec::<Vec<f32>>::new();
        // empty color
        tiles_colors.push(vec![204.0, 192.0, 179.0]);
        // 2 color
        tiles_colors.push(vec![238.0, 228.0, 218.0]);
        // 4 color
        tiles_colors.push(vec![237.0, 224.0, 200.0]);
        // 8 color
        tiles_colors.push(vec![242.0, 177.0, 121.0]);
        // 16 color
        tiles_colors.push(vec![245.0, 149.0, 99.0]);
        // 32 color
        tiles_colors.push(vec![246.0, 124.0, 95.0]);
        // 64 color
        tiles_colors.push(vec![246.0, 94.0, 59.0]);
        // 128 color
        tiles_colors.push(vec![237.0, 207.0, 114.0]);
        // 256 color
        tiles_colors.push(vec![237.0, 204.0, 97.0]);
        // 512 color
        tiles_colors.push(vec![237.0, 200.0, 80.0]);
        SettingsInJson {
            asset_folder: "bin/assets".to_string(),
            window_background_color: vec![255.0, 248.0, 239.0],
            comment1_offset_y: 72.0,
            comment2_offset_y: 100.0,
            board_padding: 12.0,
            board_offset_y: 128.0,
            tile_width: 4,
            tile_height: 4,
            tile_size: 72.0,
            tile_padding: 16.0,
            tile_background_color: vec![187.0, 173.0, 160.0],
            tiles_colors: tiles_colors,
            tile_unknow_color: vec![200.0, 0.0, 0.0],
            tile_move_time: 0.1,
            tile_new_time: 0.1,
            tile_combine_time: 0.1,
            best_rect: vec![284.0, 12.0, 96.0, 48.0,],
            score_rect: vec![176.0, 12.0, 96.0, 48.0],
            label_color: vec![187.0, 173.0, 160.0],
            button_color: vec![142.0, 122.0, 102.0],
            text_dark_color: vec![119.0, 110.0, 101.0],
            text_light_color: vec![249.0, 246.0, 242.0],
        }
    }

    pub fn load() -> SettingsInJson {
        let exe_path = current_exe();

        if exe_path.is_err() {
            return SettingsInJson::default_settings();
        }

        let mut exe_path = exe_path.unwrap();
        exe_path.pop();
        let path = exe_path.join(Path::new(SETTING_FILENAME));

        // FIXME: use this if possible  (.exists() is unstable in Rust 1.0.0)
        /*      if !path.as_path().exists() || !path.is_file() {
                    println!("Configuration file not found. Generating a default one.");
                    let default = SettingsInJson::default_settings();
                    default.save();
                    return default;
                }
                let file = File::open(&path).unwrap();
                let mut reader = BufReader::new(file);
        */
        let file = File::open(&path);

        match file {
            Err(e) => {
                println!("Configuration file can't be open ({}). Try to generate a default one.", e);
                let default = SettingsInJson::default_settings();
                default.save();
                return default;
            }
            _ => {}
        }

        let mut reader = BufReader::new(file.unwrap());
        let v: Result<SettingsInJson, Error> = serde_json::from_reader(&mut reader);
        match v {
            Ok(settings) => {
                return settings;
            }
            Err(e) => {
                println!("WARNING: Failed to read settings from file. Try to generate a default one. {}", e);
                let new_settings = SettingsInJson::default_settings();
                new_settings.save();
                return new_settings;
            }
        }
    }

    pub fn save(&self) {
        let exe_path = current_exe();

        if exe_path.is_err() {
            println!("WARNING: Failed to save settings: can't find exe path.");
            return;
        }

        let path = exe_path.unwrap();
        let file = File::create(&path.with_file_name(SETTING_FILENAME)).unwrap();
        let mut writer = BufWriter::new(file);

        match serde_json::to_string(self) {
            Ok(encoded) => {
                if let Err(e) = writer.write(encoded.as_bytes()) {
                    println!("WARNING: Failed to save settings: {}", e);
                }
            }
            Err(e) => {
                println!("WARNING: Failed to save settings: {}", e);
            }
        }
    }
}
