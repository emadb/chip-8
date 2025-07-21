use crate::constants::{FONT_MAP, FONT_MAP_SIZE, RAM_SIZE, ROM_STARTING_ADDRESS};

pub struct Mmu {
    ram: [u8; RAM_SIZE],
}

impl Mmu {
    pub fn load(rom_data: &[u8]) -> Self {
        let mut base_ram = [0; RAM_SIZE];
        let end_index = ROM_STARTING_ADDRESS + rom_data.len();
        base_ram[ROM_STARTING_ADDRESS..end_index].copy_from_slice(&rom_data);
        base_ram[..FONT_MAP_SIZE].copy_from_slice(&FONT_MAP);
        Mmu { ram: base_ram }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}
