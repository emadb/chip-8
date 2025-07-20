pub struct Stack {
    values: [u16; 12],
    sp: u8,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            sp: 0,
            values: [0; 12],
        }
    }
    pub fn push(&mut self, val: u16) {
        self.values[self.sp as usize] = val;
        self.sp += 1;
    }
    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.values[self.sp as usize]
    }
}
