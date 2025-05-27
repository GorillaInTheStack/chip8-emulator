use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUM_KEYS],
}

const START_ADDRESS: u16 = 0x200;

impl Emu {
    pub(crate) fn new() -> Self {
        let mut new_emu = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; NUM_KEYS],
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, indx: usize, pressed: bool) {
        // TODO: handle bad case
        if indx < 16 {
            self.keys[indx] = pressed;
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    // Fetch the value from our game (loaded into RAM) at the memory address stored in our Program Counter.
    // Decode this instruction.
    // Execute, which will possibly involve modifying our CPU registers or RAM.
    // Move the PC to the next instruction and repeat.
    pub fn tick(&mut self) {
        //Fetch
        let opcode = self.fetch();
        //Decode
        //Execute
        self.execute(opcode);
    }

    pub fn reset(&mut self) {
        self.program_counter = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // beep
            }
            self.sound_timer -= 1;
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    fn execute(&mut self, opcode: u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => (),
            // Clear Screen
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.program_counter = ret_addr;
            }
            // JMP NNN
            (1, _, _, _) => {
                let addr = opcode & 0xFFF;
                self.program_counter = addr;
            }
            // CALL NNN
            (2, _, _, _) => {
                let addr = opcode & 0xFFF;
                self.push(self.program_counter);
                self.program_counter = addr;
            }
            // SKIP VX == NN
            (3, _, _, _) => {
                let indx = digit2 as usize;
                let val = (opcode & 0xFF) as u8;
                if self.v_reg[indx] == val {
                    self.program_counter += 2;
                }
            }
            // SKIP VX != NN
            (4, _, _, _) => {
                let indx = digit2 as usize;
                let val = (opcode & 0xFF) as u8;
                if self.v_reg[indx] != val {
                    self.program_counter += 2;
                }
            }
            // SKIP VX == VY
            (5, _, _, 0) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                if self.v_reg[indx] == self.v_reg[indy] {
                    self.program_counter += 2;
                }
            }
            // VX = NN
            (6, _, _, _) => {
                let indx = digit2 as usize;
                let val = (opcode & 0xFF) as u8;
                self.v_reg[indx] = val;
            }
            // VX += NN
            (7, _, _, _) => {
                let indx = digit2 as usize;
                let val = (opcode & 0xFF) as u8;
                self.v_reg[indx] = self.v_reg[indx].wrapping_add(val);
            }
            // VX = VY
            (8, _, _, 0) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                self.v_reg[indx] = self.v_reg[indy];
            }
            // VX | VY
            (8, _, _, 1) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                self.v_reg[indx] |= self.v_reg[indy];
            }
            // VX & VY
            (8, _, _, 2) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                self.v_reg[indx] &= self.v_reg[indy];
            }
            // VX ^ VY
            (8, _, _, 3) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                self.v_reg[indx] ^= self.v_reg[indy];
            }
            // VX += VY
            (8, _, _, 4) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                let (new_vx, carry) = self.v_reg[indx].overflowing_add(self.v_reg[indy]);
                let new_vf = if carry { 1 } else { 0 };
                self.v_reg[indx] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX -= VY
            (8, _, _, 5) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[indx].overflowing_sub(self.v_reg[indy]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[indx] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX >>= 1
            (8, _, _, 6) => {
                let indx = digit2 as usize;
                let lsb = self.v_reg[indx] & 1;
                self.v_reg[indx] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            // VX = VY - VX
            (8, _, _, 7) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                let (new_vx, borrow) = self.v_reg[indy].overflowing_sub(self.v_reg[indx]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_reg[indx] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX <<= 1
            (8, _, _, 0xE) => {
                let indx = digit2 as usize;
                let msb = (self.v_reg[indx] >> 7) & 1;
                self.v_reg[indx] <<= 1;
                self.v_reg[0xF] = msb;
            }
            // SKIP VX != VY
            (9, _, _, 0) => {
                let indx = digit2 as usize;
                let indy = digit3 as usize;
                if self.v_reg[indx] != self.v_reg[indy] {
                    self.program_counter += 2;
                }
            }
            // I = NNN
            (0xA, _, _, _) => {
                let val = opcode & 0xFFF;
                self.i_reg = val;
            }
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let val = opcode & 0xFFF;
                self.program_counter = (self.v_reg[0] as u16) + val
            }
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let indx = digit2 as usize;
                let val = (opcode & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[indx] = rng & val;
            }
            // DRAW
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                let num_rows = digit4;

                let mut flipped = false;

                for y_line in 0..num_rows {
                    // Get base address for sprite
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    for x_line in 0..8 {
                        // Flip bits on screen according to pixel byte
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // wrap around screen
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let indx = x + SCREEN_WIDTH * y;

                            flipped |= self.screen[indx];
                            self.screen[indx] ^= true;
                        }
                    }
                }
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let indx = digit2 as usize;
                let vx = self.v_reg[indx];
                let key = self.keys[vx as usize];
                if key {
                    self.program_counter += 2;
                }
            }
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let indx = digit2 as usize;
                let vx = self.v_reg[indx];
                let key = self.keys[vx as usize];
                if !key {
                    self.program_counter += 2;
                }
            }
            // VX = DT
            (0xF, _, 0, 7) => {
                let indx = digit2 as usize;
                self.v_reg[indx] = self.delay_timer;
            }
            // WAIT KEY (BLOCKING)
            (0xF, _, 0, 0xA) => {
                let indx = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[indx] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.program_counter -= 2;
                }
            }
            // DT = VX
            (0xF, _, 1, 5) => {
                let indx = digit2 as usize;
                self.delay_timer = self.v_reg[indx];
            }
            // ST = VX
            (0xF, _, 1, 8) => {
                let indx = digit2 as usize;
                self.sound_timer = self.v_reg[indx];
            }
            // I += VX
            (0xF, _, 1, 0xE) => {
                let indx = digit2 as usize;
                let vx = self.v_reg[indx] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            }
            // I = FONT
            (0xF, _, 2, 9) => {
                let indx = digit2 as usize;
                let sprite = self.v_reg[indx] as u16;
                self.i_reg = sprite * 5;
            }
            // BCD
            (0xF, _, 3, 3) => {
                let indx = digit2 as usize;
                // TODO: optimize
                let vx = self.v_reg[indx] as f32;
                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }
            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let indx = digit2 as usize;
                let i = self.i_reg as usize;
                for indy in 0..=indx {
                    self.ram[i + indy] = self.v_reg[indy];
                }
            }
            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let indx = digit2 as usize;
                let i = self.i_reg as usize;
                for indy in 0..=indx {
                    self.v_reg[indy] = self.ram[i + indy];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode {}", opcode),
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[self.program_counter as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        opcode
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
