use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub type Screen = [[u8; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];

pub struct Display {
    screen: Screen,
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: [[0; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
        }
    }

    pub fn draw(&mut self, x_coord: u16, y_coord: u16, sprite: Vec<u8>) -> bool {
        let mut pixel_erased = false;
        for (row_index, sprite_byte) in sprite.iter().enumerate() {
            let current_y = (y_coord as usize + row_index) % SCREEN_HEIGHT as usize;

            for bit_index in 0..8 {
                let current_x = (x_coord as usize + bit_index) % SCREEN_WIDTH as usize;
                let sprite_pixel = (sprite_byte >> (7 - bit_index)) & 0x01;
                let screen_pixel = self.screen[current_x][current_y];
                self.screen[current_x][current_y] ^= sprite_pixel;

                if screen_pixel == 1 && sprite_pixel == 1 {
                    pixel_erased = true;
                }
            }
        }
        pixel_erased
    }

    pub fn clear(&mut self) {
        for row in self.screen.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
    }

    pub fn get_pixels(&self) -> Screen {
        self.screen
    }
}