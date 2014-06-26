
use graphics::*;
use piston::{
    AssetStore,
};
use opengl_graphics::{
    Gl,
    Texture,
};

static DIGITS_WIDTH: f64 = 20.0;
static DIGITS_HEIGHT: f64 = 26.0;

pub struct NumberRenderer {
    image: Texture,
}

impl NumberRenderer {
    pub fn new(asset_store: &AssetStore) -> NumberRenderer {
        NumberRenderer {
            image: Texture::from_path(&asset_store.path("digits.png").unwrap()).unwrap(),
        }
    }

    pub fn render(&self, number: u32, center_x: f64, center_y: f64, max_width: f64,
                  color: [f32, ..3], c: &Context, gl: &mut Gl) {
        let digits = number_to_digits(number);
        let total_width = DIGITS_WIDTH * digits.len() as f64;
        let total_width = if total_width > max_width {
            max_width
        } else {
            total_width
        };
        let mut x = center_x - total_width / 2.0;
        let width = total_width / digits.len() as f64;
        let height = width / DIGITS_WIDTH * DIGITS_HEIGHT;
        let y = center_y - height / 2.0;

        for digit in digits.iter() {
            c.rect(x, y, width, height)
             .image(&self.image)
             .src_rect((*digit * DIGITS_WIDTH as u32) as i32, 0, DIGITS_WIDTH as i32, DIGITS_HEIGHT as i32)
             .rgba(color[0], color[1], color[2], 1.0)
             .draw(gl);
            x += width;
        }
    }
}

fn number_to_digits(number: u32) -> Vec<u32> {
    let mut digits = Vec::<u32>::new();
    if number == 0 {
        digits.push(0);
        return digits;
    }

    let mut n = number;
    while n != 0 {
        digits.unshift(n % 10);
        n /= 10;
    }
    digits
}

