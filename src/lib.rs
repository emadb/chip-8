pub mod constants;
pub mod cpu;
pub mod display;
pub mod keypad;
pub mod mmu;
pub mod stack;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use cpu::Cpu;
use display::Display;
use keypad::Keypad;
use mmu::Mmu;

use std::fs;

pub struct Chip8 {
    cpu: Cpu,
    memory: Mmu,
    display: Display,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new(rom_data: &[u8]) -> Self {
        Chip8 {
            cpu: Cpu::new(),
            memory: Mmu::load(rom_data),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn load(rom_file: &str) -> Chip8 {
        let rom_data = fs::read(rom_file).unwrap();
        Chip8::new(&rom_data)
    }

    pub fn update_timers(&mut self) {
        self.cpu.tick_timers();
    }

    pub fn press_key(&mut self, key: u8) {
        self.keypad.press_key(key);
    }

    pub fn clear_key(&mut self) {
        self.keypad.clear();
    }

    pub fn update(&mut self) {
        self.cpu
            .tick(&mut self.memory, &mut self.display, &self.keypad);
    }

    pub fn get_pixels(&self) -> [[u8; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize] {
        self.display.get_pixels()
    }
}
