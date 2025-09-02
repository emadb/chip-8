use crate::{display::Display, keypad::Keypad, mmu::Mmu};

pub struct Bus {
    pub memory: Mmu,
    pub display: Display,
    pub keypad: Keypad,
}

impl Bus {
    pub fn new(memory: Mmu, display: Display, keypad: Keypad) -> Self {
        Self {
            memory,
            display,
            keypad,
        }
    }
}
