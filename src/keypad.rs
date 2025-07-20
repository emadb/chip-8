pub struct Keypad {
    keys: [bool; 16],
    last_key_pressed: Option<u8>,
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
            last_key_pressed: None,
        }
    }

    pub fn get_key(&self, i: u8) -> bool {
        self.keys[i as usize]
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.last_key_pressed
    }

    pub fn press_key(&mut self, key: u8) {
        self.keys[key as usize] = true;
        self.last_key_pressed = Some(key);
    }

    pub fn clear(&mut self) {
        for k in 0..16 {
            self.keys[k] = false;
        }
        self.last_key_pressed = None;
    }
}
