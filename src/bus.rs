use crate::{
    display::{Display, Screen},
    keypad::Keypad,
    mmu::Mmu,
};

pub struct Bus {
    memory: Mmu,
    display: Display,
    keypad: Keypad,
}

impl Bus {
    pub fn new(memory: Mmu, display: Display, keypad: Keypad) -> Self {
        Self {
            memory,
            display,
            keypad,
        }
    }

    pub fn press_key(&mut self, key: u8) {
        self.keypad.press_key(key);
    }

    pub fn get_key(&self, i: u8) -> bool {
        self.keypad.get_key(i)
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.keypad.get_key_pressed()
    }

    pub fn reset_keypad(&mut self) {
        self.keypad.reset();
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory.write_byte(addr, value);
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory.read_byte(addr)
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    pub fn draw(&mut self, x: u16, y: u16, sprite: Vec<u8>) -> bool {
        self.display.draw(x, y, sprite)
    }

    pub fn get_pixels(&self) -> Screen {
        self.display.get_pixels()
    }
}
