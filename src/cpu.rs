use rand::Rng;

use crate::{memory::Memory, display::Display, display::DisplayDriver, chip8, keyboard::Keyboard, instructions::Instruction};

pub type Address = u16;
pub type Register = u8;
pub type Value = u8;

pub struct Cpu {
    v_registers: [Register; 16],
    i_register: Address,
    delay_timer: Value,
    sound_timer: Value,
    program_counter: Address,
    stack: Vec<Address>,
    rng: rand::rngs::ThreadRng,
}

impl Cpu {
    pub fn new(memory_start: Address) -> Cpu {
        Cpu {
            v_registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: memory_start,
            stack: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn i_register(&self) -> Address {
        self.i_register
    }

    pub fn v_registers(&self) -> &[Value; 16] {
        &self.v_registers
    }

    pub fn program_counter(&self) -> Address {
        self.program_counter
    }

    pub fn cycle<T: DisplayDriver>(&mut self, memory: &mut Memory, display: &mut Display<T>, keyboard: &Keyboard) {
        // fetch
        let opcode = memory.read_word(self.program_counter);
        self.program_counter += 2;

        // decode
        // Option here to decode entire program upfront? (although some programs may self modify)
        let instruction = Instruction::decode(opcode);

        // execute
        match instruction {
            Instruction::ClearScreen => {
                display.clear();
            },
            Instruction::Jump(address) => {
                self.program_counter = address;
            },
            Instruction::Return => {
                self.program_counter = self.stack.pop().unwrap();
            },
            Instruction::Call(address) => {
                self.stack.push(self.program_counter);
                self.program_counter = address;
            },
            Instruction::CheckEqualValue(vx, value) => {
                if self.v_registers[vx as usize] == value {
                    self.program_counter += 2;
                }
            },
            Instruction::CheckNotEqualValue(vx, value) => {
                if self.v_registers[vx as usize] != value {
                    self.program_counter += 2;
                }
            },
            Instruction::CheckEqual(vx, vy) => {
                if self.v_registers[vx as usize] == self.v_registers[vy as usize] {
                    self.program_counter += 2;
                }
            },
            Instruction::CheckNotEqual(vx, vy) => {
                if self.v_registers[vx as usize] != self.v_registers[vy as usize] {
                    self.program_counter += 2;
                }
            },
            Instruction::SetRegisterToValue(register, value) => {
                self.v_registers[register as usize] = value;
            },
            Instruction::AddValueToRegister(register, value) => {
                self.v_registers[register as usize] = self.v_registers[register as usize].wrapping_add(value);
            },
            Instruction::SetRegister(vx, vy) => {
                self.v_registers[vx as usize] = self.v_registers[vy as usize];
            },
            Instruction::Or(vx, vy) => {
                self.v_registers[vx as usize] |= self.v_registers[vy as usize];
            },
            Instruction::And(vx, vy) => {
                self.v_registers[vx as usize] &= self.v_registers[vy as usize];
            },
            Instruction::Xor(vx, vy) => {
                self.v_registers[vx as usize] ^= self.v_registers[vy as usize];
            },
            Instruction::Add(vx, vy) => {
                let (val, overflow) = self.v_registers[vx as usize].overflowing_add(self.v_registers[vy as usize]);

                self.v_registers[vx as usize] = val;
                self.v_registers[0xF] = overflow as u8;
            },
            Instruction::Subtract{ destination, first, second} => {
                let (val, overflow) = self.v_registers[first as usize].overflowing_sub(self.v_registers[second as usize]);

                self.v_registers[destination as usize] = val;
                self.v_registers[0xF] = !overflow as u8;
            },
            Instruction::ShiftLeft(vx) => {
                let lost_bit = (self.v_registers[vx as usize] >> 7) & 0x1;

                self.v_registers[vx as usize] <<= 1;
                self.v_registers[0xF] = lost_bit;
            },
            Instruction::ShiftRight(vx) => {
                let lost_bit = self.v_registers[vx as usize] & 0x01;
        
                self.v_registers[vx as usize] >>= 1;
                self.v_registers[0xF] = lost_bit;
            },
            Instruction::SetIndex(index) => {
                self.i_register = index;
            },
            Instruction::Display { vx, vy, pixel_height } => {
                let x = self.v_registers[vx as usize];
                let y = self.v_registers[vy as usize];
                self.v_registers[0xF] = 0;

                display.begin_draw();

                for i in 0..pixel_height as u16 {
                    let pixel_location = self.i_register + i;
                    let pixels = memory.read_byte(pixel_location);
                    println!("Address: {:02x}", pixel_location);
                    println!("Pixel: {:02x}", pixels);

                    display.set_pixels(x, y.saturating_add(i as u8), pixels, y);
                }

                if display.did_collide() {
                    self.v_registers[0xF] = 1;
                }
            },
            Instruction::Random(vx, value) => {
                let random_byte = self.rng.gen::<u8>();
                self.v_registers[vx as usize] = random_byte & value;
            },
            Instruction::SkipIfKeyPressed(x) => {
                if keyboard.is_key_pressed(self.v_registers[x as usize]) {
                    self.program_counter += 2;
                }
            },
            Instruction::SkipIfKeyNotPressed(x) => {
                if !keyboard.is_key_pressed(self.v_registers[x as usize]) {
                    self.program_counter += 2;
                }
            },
            Instruction::ReadDelayTimer(x) => {
                self.v_registers[x as usize] = self.delay_timer;
            },
            Instruction::WriteDelayTimer(vx) => {
                self.delay_timer = self.v_registers[vx as usize];
            },
            Instruction::WriteSoundTimer(vx) => {
                self.sound_timer = self.v_registers[vx as usize]; 
            },
            Instruction::AddRegisterToIndex(vx) => {
                let value = self.v_registers[vx as usize];
                self.i_register += value as u16;

                // Not clear if this is supposed to wrap or carry on here
                if self.i_register > 0xFFF {
                    self.v_registers[0xF] = 1;
                }
            },
            Instruction::WaitForKeyPress(vx) => {
                if keyboard.has_signal_keypress() {
                    self.v_registers[vx as usize] = keyboard.key_last_pressed().unwrap();
                } else {
                    self.program_counter -= 2;
                }
            },
            Instruction::SetIndexToSprite(vx) => {
                let value = self.v_registers[vx as usize];
                self.i_register = chip8::FONT_START + (value as u16 * 5);
            },
            Instruction::StoreBCD(vx) => {
                let value = self.v_registers[vx as usize];
                    
                let hundreds = (value / 100) as u8;
                let tens = ((value % 100) / 10) as u8;
                let ones = (value % 10) as u8;

                memory.write_byte(self.i_register, hundreds);
                memory.write_byte(self.i_register + 1, tens);
                memory.write_byte(self.i_register + 2, ones);
            },
            Instruction::StoreRegisters(vx) => {
                let index = self.i_register;

                for i in 0..=vx {
                    let value = self.v_registers[i as usize];
                    memory.write_byte(index + i as u16, value);
                }
            },
            Instruction::LoadRegisters(vx) => {
                let index = self.i_register;

                for i in 0..=vx {
                    let value = memory.read_byte(index + i as u16);
                    self.v_registers[i as usize] = value;
                }
            },
        }
    }

    pub fn timer_cycle(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }
}

// impl fmt::Debug for Memory {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ data: {:?} }}", self.data)
//     }
// }
