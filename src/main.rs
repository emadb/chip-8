use chip_8::Chip8;
use chip_8::constants::{
    CHIP8_UPDATE_INTERVAL, SCALE_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH, TICKS_PER_FRAME,
};
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use macroquad::{color, shapes::draw_rectangle};
use std::env;
use std::process::exit;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Chip-8"),
        window_width: SCALE_FACTOR as i32 * SCREEN_WIDTH as i32,
        window_height: SCALE_FACTOR as i32 * SCREEN_HEIGHT as i32,
        window_resizable: false,
        platform: Platform {
            swap_interval: Some(30),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // let mut chip8 = chip_8::load("roms/2-ibm-logo.ch8");

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("usage: chip-8 path/to/rom_file");
        exit(0);
    }

    let mut chip8 = Chip8::load(&args[1]);

    let mut last_update_time = get_time();
    loop {
        clear_background(BLACK);

        if let Some(key) = check_keypad() {
            chip8.press_key(key);
        } else {
            chip8.clear_key();
        }

        if get_time() - last_update_time > CHIP8_UPDATE_INTERVAL {
            last_update_time = get_time();
            for _ in 0..TICKS_PER_FRAME {
                chip8.update();
            }
            chip8.update_timers();
        }
        let pixels = chip8.get_pixels();
        render(pixels);
        next_frame().await;
    }
}

fn render(pixels: [[u8; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize]) {
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let color = if pixels[x as usize][y as usize] == 0 {
                color::BLACK
            } else {
                color::WHITE
            };
            draw_rectangle(
                x as f32 * SCALE_FACTOR,
                y as f32 * SCALE_FACTOR,
                SCALE_FACTOR,
                SCALE_FACTOR,
                color,
            );
        }
    }
}

fn check_keypad() -> Option<u8> {
    if is_key_down(KeyCode::Key1) {
        return Some(0x1);
    }
    if is_key_down(KeyCode::Key2) {
        return Some(0x2);
    }
    if is_key_down(KeyCode::Key3) {
        return Some(0x3);
    }
    if is_key_down(KeyCode::Key4) {
        return Some(0xC);
    }
    if is_key_down(KeyCode::Q) {
        return Some(0x4);
    }
    if is_key_down(KeyCode::W) {
        return Some(0x5);
    }
    if is_key_down(KeyCode::E) {
        return Some(0x6);
    }
    if is_key_down(KeyCode::R) {
        return Some(0xD);
    }
    if is_key_down(KeyCode::A) {
        return Some(0x7);
    }
    if is_key_down(KeyCode::S) {
        return Some(0x8);
    }
    if is_key_down(KeyCode::D) {
        return Some(0x9);
    }
    if is_key_down(KeyCode::F) {
        return Some(0xE);
    }
    if is_key_down(KeyCode::Z) {
        return Some(0xA);
    }
    if is_key_down(KeyCode::X) {
        return Some(0x0);
    }
    if is_key_down(KeyCode::C) {
        return Some(0xB);
    }
    if is_key_down(KeyCode::V) {
        return Some(0xF);
    }
    None
}
