pub mod bus;
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

use crate::bus::Bus;

pub struct Chip8 {
    cpu: Cpu,
    bus: Bus,
}

impl Chip8 {
    pub fn new(rom_data: &[u8]) -> Self {
        let bus = Bus::new(Mmu::load(rom_data), Display::new(), Keypad::new());

        Chip8 {
            cpu: Cpu::new(),
            bus: bus,
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
        self.bus.press_key(key);
    }

    pub fn clear_key(&mut self) {
        self.bus.reset_keypad();
    }

    pub fn update(&mut self) {
        self.cpu.tick(&mut self.bus);
    }

    pub fn get_pixels(&self) -> [[u8; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize] {
        self.bus.get_pixels()
    }
}
