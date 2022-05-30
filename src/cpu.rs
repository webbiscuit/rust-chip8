use rand::Rng;

use crate::{memory::Memory, display::Display, display::DisplayDriver, chip8};

pub struct Cpu {
    v_registers: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack: Vec<u16>,
    rng: rand::rngs::ThreadRng, // Doesn't belong in cpu?
    key_pressed: Option<u8> // Doesn't belong in cpu
}

impl Cpu {
    pub fn new(memory_start: u16) -> Cpu {
        Cpu {
            v_registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: memory_start,
            stack: Vec::new(),
            rng: rand::thread_rng(),
            key_pressed: None
        }
    }

    pub fn i_register(&self) -> u16 {
        self.i_register
    }

    pub fn v_registers(&self) -> &[u8; 16] {
        &self.v_registers
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn cycle<T: DisplayDriver>(&mut self, memory: &mut Memory, display: &mut Display<T>) {
        // fetch
        let opcode = memory.read_word(self.program_counter);
        self.program_counter += 2;

        // println!("{:04x}", opcode);

        match opcode & 0xF000
        {
            0x0000 => {
                match opcode
                {
                    0x00E0 => {
                        // 0x00E0: Clears the screen.
                        println!("{:#06X} - Clears the screen.", opcode);
                        display.clear();
                    },
                    0x00EE => {
                        // 0x00EE: Pop PC off the stack.
                        println!("{:#06X} - Pop PC off the stack.", opcode);
                        self.program_counter = self.stack.pop().unwrap();
                    },
                    _ => {
                        panic!("Unknown opcode: {:#06X}", opcode);
                    }
                }
            },
            0x1000 => {
                // 0x1NNN: Jumps to address NNN.
                let address = opcode & 0x0FFF;

                println!("{:#06X} - Jump to address {}", opcode, address);

                self.program_counter = address;
            },
            0x2000 => {
                // 0x2NNN: Call subroutine at address NNN.
                let address = opcode & 0x0FFF;

                println!("{:#06X} - Jump to subroutine {}", opcode, address);

                self.stack.push(self.program_counter);
                self.program_counter = address;
            },
            0x3000 => {
                // eq
                let address = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                println!("{:#06X} - V{:X} eq {}", opcode, address, value);

                if self.v_registers[address as usize] == value {
                    self.program_counter += 2;
                }
            },
            0x4000 => {
                // neq
                let address = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                println!("{:#06X} - V{:X} neq {}", opcode, address, value);

                if self.v_registers[address as usize] != value {
                    self.program_counter += 2;
                }
            },
            0x5000 => {
                // eq register
                let address1 = ((opcode & 0x0F00) >> 8) as u8;
                let address2 = ((opcode & 0x00F0) >> 4) as u8;

                println!("{:#06X} - V{:X} eq V{:X}", opcode, address1, address2);

                if self.v_registers[address1 as usize] == self.v_registers[address2 as usize] {
                    self.program_counter += 2;
                }
            },
            0x6000 => {
                let ix = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                println!("{:#06X} Set V{:X} to {:#04X}.", opcode, ix, value);
                self.v_registers[ix] = value;
            },
            0x7000 => { 
                let ix = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;

                println!("{:#06X} Add {:#04X} to V{:X}.", opcode, value, ix);

                self.v_registers[ix] = self.v_registers[ix].saturating_add(value);
            },
            0x8000 => {
                match opcode & 0x000F
                {
                    0x0000 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} Set V{:X} to V{:X}.", opcode, vx, vy);
                        self.v_registers[vx as usize] = self.v_registers[vy as usize];
                    },
                    0x0001 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} OR V{:X} to V{:X}.", opcode, vx, vy);
                        self.v_registers[vx as usize] |= self.v_registers[vy as usize];
                    },
                    0x0002 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} AND V{:X} to V{:X}.", opcode, vx, vy);
                        self.v_registers[vx as usize] &= self.v_registers[vy as usize];
                    },
                    0x0003 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} XOR V{:X} to V{:X}.", opcode, vx, vy);
                        self.v_registers[vx as usize] ^= self.v_registers[vy as usize];
                    },
                    0x0004 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} Sub VX (V{:X}) - VY (V{:X}).", opcode, vx, vy);

                        let (val, overflow) = self.v_registers[vx as usize].overflowing_add(self.v_registers[vy as usize]);

                        self.v_registers[vx as usize] = val;
                        self.v_registers[0xF] = overflow as u8;
                    },
                    0x0005 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} Sub VX (V{:X}) - VY (V{:X}).", opcode, vx, vy);

                        let (val, overflow) = self.v_registers[vx as usize].overflowing_sub(self.v_registers[vy as usize]);
                        
                        self.v_registers[vx as usize] = val;
                        self.v_registers[0xF] = !overflow as u8;
                    },
                    0x0006 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let lost_bit = (self.v_registers[vx as usize] & 0x0001) as u8;

                        println!("{:#06X} >>1 V{:X}.", opcode, vx);

                        self.v_registers[vx as usize] >>= 1;
                        self.v_registers[0xF] = lost_bit;

                    },
                    0x0007 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        println!("{:#06X} Sub VY (V{:X}) - VX (V{:X}).", opcode, vx, vy);

                        let (val, overflow) = self.v_registers[vy as usize].overflowing_sub(self.v_registers[vx as usize]);
                        self.v_registers[vx as usize] = val;
                        self.v_registers[0xF] = !overflow as u8;
                    },
                    0x000E => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let lost_bit = ((self.v_registers[vx as usize] as u16 & 0x1000) >> 12) as u8;

                        println!("{:#06X} <<1 V{:X}.", opcode, vx);

                        self.v_registers[vx as usize] <<= 1;
                        self.v_registers[0xF] = lost_bit;

                    },
                    _ => {
                        panic!("Unknown opcode: {:#06X}", opcode);
                    }
                }
            },
            0x9000 => {
                // neq register
                let address1 = ((opcode & 0x0F00) >> 8) as u8;
                let address2 = ((opcode & 0x00F0) >> 4) as u8;

                println!("{:#06X} - V{:X} neq V{:X}", opcode, address1, address2);

                if self.v_registers[address1 as usize] != self.v_registers[address2 as usize] {
                    self.program_counter += 2;
                }
            },
            0xA000 => {
                let address = (opcode & 0x0FFF) as u16;
                println!("{:#06X} - Set the I Reg {:#04X}.", opcode, address);
                self.i_register = address;
            },
            0xC000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let random_byte = self.rng.gen::<u8>();

                let value = (opcode & 0x00FF) as u8;

                println!("{:#06X} - Set V{:X} to a random byte and AND it with {:X}.", opcode, vx, value);

                self.v_registers[vx as usize] = random_byte & value;
            },
            0xD000 => {
                let x = self.v_registers[((opcode & 0x0F00) >> 8) as usize];
                let y = self.v_registers[((opcode & 0x00F0) >> 4) as usize];
                let n = (opcode & 0x000F) as u16;
                println!("{:#06X} - Draw sprite at ({}, {}) with {} bytes.", opcode, x, y, n);

                self.v_registers[0xF] = 0;

                for i in 0..n {
                    let pixel_location = self.i_register + i;
                    let pixels = memory.read_byte(pixel_location);
                    println!("Address: {:02x}", pixel_location);
                    println!("Pixel: {:02x}", pixels);

                    display.set_pixels(x, y.saturating_add(i as u8), pixels);
                }
            },
            0xE000 => {
                match opcode & 0x00FF
                {
                    0x009E => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        println!("{:#06X} - Skip next instruction if key with value V{:X} is pressed.", opcode, vx);

                        if let Some(key_pressed) = self.key_pressed {
                            if key_pressed == self.v_registers[vx as usize] {
                                self.program_counter += 2;
                            }
                        }                    
                    },
                    0x00A1 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        println!("{:#06X} - Skip next instruction if key with value V{:X} is not pressed.", opcode, vx);

                        if self.key_pressed == None {
                            self.program_counter += 2;
                        }
                        else if let Some(key_pressed) = self.key_pressed {
                            if key_pressed != self.v_registers[vx as usize] {
                                self.program_counter += 2;
                            }
                        }                    
                    },
                    _ => {
                        panic!("Unknown opcode: {:#06X}", opcode);
                    }
                }
            },
            0xF000 => {
                match opcode & 0x00FF
                {
                    0x0007 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let delay_timer = self.delay_timer;

                        println!("{:#06X} - Set V{:X} to DT {}.", opcode, vx, delay_timer);
                        self.v_registers[vx as usize] = delay_timer;
                    },
                    0x0015 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Set DT to value in V{:X}, {}.", opcode, vx, value);
                        self.delay_timer = value;
                    },
                    0x0018 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Set ST to value in V{:X}, {}.", opcode, vx, value);
                        self.sound_timer = value;
                    },
                    0x001E => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Set I to I + V{:X}, {}.", opcode, vx, value);
                        self.i_register += value as u16;

                        if self.i_register > 0xFFF {
                            self.v_registers[0xF] = 1;
                        }
                    },
                    0x000A => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Wait for key press, store key in V{:X}, {}.", opcode, vx, value);

                        if self.key_pressed == None {
                            self.program_counter -= 2;
                        }
                        else if let Some(key_pressed) = self.key_pressed {
                            self.v_registers[vx as usize] = key_pressed;
                        }
                    },
                    0x0029 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Set I to the location of the sprite for the character in V{:X}, {}.", opcode, vx, value);
                        self.i_register = chip8::FONT_START + (value as u16 * 5);
                    },
                    0x0033 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let value = self.v_registers[vx as usize];

                        println!("{:#06X} - Store BCD representation of V{:X} in memory at I.", opcode, vx);

                        let hundreds = (value / 100) as u8;
                        let tens = ((value % 100) / 10) as u8;
                        let ones = (value % 10) as u8;

                        memory.write_byte(self.i_register, hundreds);
                        memory.write_byte(self.i_register + 1, tens);
                        memory.write_byte(self.i_register + 2, ones);

                    },
                    0x0055 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let index = self.i_register;

                        println!("{:#06X} - Store to V0-V{:X} memory, starting {:X}", opcode, vx, index);

                        for i in 0..(vx + 1) {
                            let value = self.v_registers[i as usize];
                            memory.write_byte(index + i as u16, value);
                        }
                    },
                    0x0065 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;

                        let index = self.i_register;

                        println!("{:#06X} - Read from V0-V{:X} memory, starting {:X}", opcode, vx, index);

                        for i in 0..(vx + 1) {
                            let byte = memory.read_byte(index + i as u16);
                            self.v_registers[i as usize] = byte;
                        }
                    },
                    _ => {
                        panic!("Unknown opcode: {:#06X}", opcode);
                    }
                }
            },
            _ => {
                panic!("Unknown opcode: {:#06X}", opcode);
            }
        }

        // decode
        // execute
    }

    pub fn timer_cycle(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    pub fn key_pressed(&mut self, key: u8) {
        self.key_pressed = Some(key);
    }

    pub(crate) fn key_up(&mut self) {
        self.key_pressed = None;
    }


}

// impl fmt::Debug for Memory {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ data: {:?} }}", self.data)
//     }
// }
