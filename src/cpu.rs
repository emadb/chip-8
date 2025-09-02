use crate::{display::Display, keypad::Keypad, mmu::Mmu, stack::Stack};
use rand::random;

pub struct Cpu {
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: Stack,
    dt: u8,
    st: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x200,
            v: [0; 16],
            i: 0,
            stack: Stack::new(),
            dt: 0,
            st: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Mmu, display: &mut Display, keypad: &Keypad) -> () {
        let opcode = self.fetch_op(memory);
        self.execute_instruction(opcode, memory, display, keypad);
    }

    fn fetch_op(&mut self, memory: &Mmu) -> u16 {
        let hb = memory.read_byte(self.pc) as u16;
        let lb = memory.read_byte(self.pc + 1) as u16;
        self.pc += 2;
        ((hb << 8) | lb) as u16
    }

    fn execute_instruction(
        &mut self,
        op: u16,
        memory: &mut Mmu,
        display: &mut Display,
        keypad: &Keypad,
    ) {
        let digits = nibbles(op);

        match digits {
            (0x0, 0x0, 0xE, 0x0) => self.cls(display),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, a, b, c) => self.jmp(compose3(a, b, c)),
            (0x2, a, b, c) => self.call(compose3(a, b, c)),
            (0x3, x, b, c) => self.se_vx_byte(x, b << 4 | c),
            (0x4, x, b, c) => self.sne_vx_byte(x, b << 4 | c),
            (0x5, x, y, 0x0) => self.se_vx_vy(x, y),
            (0x6, a, b, c) => self.ld_vx_byte(a, b << 4 | c),
            (0x7, a, b, c) => self.add_vx_byte(a, b << 4 | c),
            (0x8, x, y, 0x0) => self.ld_vx_vy(x, y),
            (0x8, x, y, 0x1) => self.or_vx_vy(x, y),
            (0x8, x, y, 0x2) => self.and_vx_vy(x, y),
            (0x8, x, y, 0x3) => self.xor_vx_vy(x, y),
            (0x8, x, y, 0x4) => self.add_vx_vy(x, y),
            (0x8, x, y, 0x5) => self.sub_vx_vy(x, y),
            (0x8, x, _, 0x6) => self.shr_vx(x),
            (0x8, x, y, 0x7) => self.sub_vy_vx(x, y),
            (0x8, x, _, 0xE) => self.shl_vx(x),
            (0x9, x, y, 0) => self.sne_vx_vy(x, y),
            (0xA, a, b, c) => self.set_i(compose3(a, b, c)),
            (0xB, a, b, c) => self.jp_v0(compose3(a, b, c)),
            (0xC, x, b, c) => self.rnd_vx(x, b << 4 | c),
            (0xD, x, y, n) => self.drw(x, y, n, memory, display),
            (0xE, x, 0x9, 0xE) => self.skip_vx(x, keypad),
            (0xE, x, 0xA, 0x1) => self.skip_nvx(x, keypad),
            (0xF, x, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, x, 0x0, 0xA) => self.ld_vx_k(x, keypad),
            (0xF, x, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, x, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, x, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, x, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, x, 0x3, 0x3) => self.ld_b_vx(x, memory),
            (0xF, x, 0x5, 0x5) => self.ld_i_vx(x, memory),
            (0xF, x, 0x6, 0x5) => self.ld_vx_i(x, memory),
            (_, _, _, _) => unimplemented!("Bad opcode: {}", op),
        }
    }

    fn cls(&self, display: &mut Display) {
        display.clear();
    }

    fn ret(&mut self) {
        let addr = self.stack.pop();
        self.pc = addr;
    }

    fn jmp(&mut self, addr: u16) {
        self.pc = addr as u16;
    }

    fn call(&mut self, addr: u16) {
        self.stack.push(self.pc);
        self.pc = addr;
    }

    fn se_vx_byte(&mut self, x: u8, val: u8) {
        if self.v[x as usize] == val {
            self.pc += 2;
        }
    }

    fn sne_vx_byte(&mut self, x: u8, val: u8) {
        if self.v[x as usize] != val {
            self.pc += 2;
        }
    }

    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    fn ld_vx_byte(&mut self, x: u8, val: u8) {
        self.v[x as usize] = val;
    }

    fn add_vx_byte(&mut self, x: u8, val: u8) {
        let current_vx = self.v[x as usize];
        self.v[x as usize] = current_vx.wrapping_add(val);
    }

    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
    }

    fn or_vx_vy(&mut self, x: u8, y: u8) {
        let res = self.v[x as usize] | self.v[y as usize];
        self.v[x as usize] = res;
    }

    fn and_vx_vy(&mut self, x: u8, y: u8) {
        let res = self.v[x as usize] & self.v[y as usize];
        self.v[x as usize] = res;
    }

    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        let res = self.v[x as usize] ^ self.v[y as usize];
        self.v[x as usize] = res;
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let (res, carry) = self.v[x as usize].overflowing_add(self.v[y as usize]);
        self.v[x as usize] = res;
        self.v[0xF] = bool_to_bit(carry);
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        let (res, borrow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
        self.v[x as usize] = res;
        self.v[0xF] = bool_to_bit(!borrow);
    }

    fn shr_vx(&mut self, x: u8) {
        let lsb = self.v[x as usize] & 1;
        self.v[x as usize] >>= 1;
        self.v[0xF] = lsb;
    }

    fn sub_vy_vx(&mut self, x: u8, y: u8) {
        let (res, borrow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
        self.v[y as usize] = res;
        self.v[0xF] = bool_to_bit(!borrow);
    }

    fn shl_vx(&mut self, x: u8) {
        let msb = self.v[x as usize] & 0x80;
        self.v[x as usize] <<= 1;
        self.v[0xF] = msb;
    }
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2
        }
    }

    fn set_i(&mut self, val: u16) {
        self.i = val;
    }

    fn jp_v0(&mut self, val: u16) {
        self.pc = self.v[0x00].wrapping_add(val as u8) as u16;
    }

    fn rnd_vx(&mut self, x: u8, v: u8) {
        let rnd: u8 = random();
        self.v[x as usize] = rnd & v;
    }

    fn drw(&mut self, x: u8, y: u8, n: u8, mem: &Mmu, display: &mut Display) {
        let x = self.v[x as usize] as u16;
        let y = self.v[y as usize] as u16;

        let mut sprite: Vec<u8> = Vec::new();

        for y_line in 0..n {
            let addr = self.i + y_line as u16;
            let pixels = mem.read_byte(addr);
            sprite.push(pixels);
        }

        let flipped = display.draw(x, y, sprite);

        if flipped {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
    }

    fn skip_vx(&mut self, x: u8, keypad: &Keypad) {
        if keypad.get_key(self.v[x as usize]) {
            self.pc += 2;
        }
    }

    fn skip_nvx(&mut self, x: u8, keypad: &Keypad) {
        if !keypad.get_key(self.v[x as usize]) {
            self.pc += 2;
        }
    }

    fn ld_vx_dt(&mut self, x: u8) {
        self.v[x as usize] = self.dt;
    }

    fn ld_vx_k(&mut self, x: u8, keypad: &Keypad) {
        let key_pressed = keypad.get_key_pressed();
        if let Some(key) = key_pressed {
            self.v[x as usize] = key;
        } else {
            self.pc -= 2;
        }
    }

    fn ld_dt_vx(&mut self, x: u8) {
        self.dt = self.v[x as usize];
    }

    fn ld_st_vx(&mut self, x: u8) {
        self.st = self.v[x as usize];
    }

    fn add_i_vx(&mut self, x: u8) {
        self.i = self.i.wrapping_add(self.v[x as usize] as u16);
    }

    fn ld_f_vx(&mut self, x: u8) {
        let font = self.v[x as usize] as u16;
        self.i = font * 5;
    }

    fn ld_b_vx(&self, x: u8, memory: &mut Mmu) {
        let vx = self.v[x as usize];
        let hundreds = vx / 100;
        let tens = (vx % 100) / 10;
        let ones = vx % 10;

        memory.write_byte(self.i, hundreds);
        memory.write_byte(self.i + 1, tens);
        memory.write_byte(self.i + 2, ones);
    }

    fn ld_i_vx(&self, x: u8, memory: &mut Mmu) {
        for v in 0..=x {
            memory.write_byte(self.i + v as u16, self.v[v as usize]);
        }
    }

    fn ld_vx_i(&mut self, x: u8, memory: &mut Mmu) {
        for v in 0..=x {
            self.v[v as usize] = memory.read_byte(self.i + v as u16);
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                actually_beep::beep_with_hz_and_millis(880, 110).unwrap();
            }
            self.st -= 1;
        }
    }
}

fn compose3(a: u8, b: u8, c: u8) -> u16 {
    let part_a = (a as u16) << 8;
    let part_b = (b as u16) << 4;
    let part_c = c as u16;

    (part_a | part_b | part_c) as u16
}
fn bool_to_bit(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

fn nibbles(opcode: u16) -> (u8, u8, u8, u8) {
    let d1 = ((opcode >> 12) & 0xF) as u8;
    let d2 = ((opcode >> 8) & 0xF) as u8;
    let d3 = ((opcode >> 4) & 0xF) as u8;
    let d4 = (opcode & 0xF) as u8;

    (d1, d2, d3, d4)
}
