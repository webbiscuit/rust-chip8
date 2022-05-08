use std::fmt;

use crate::{memory::Memory, display::Display, display::DisplayDriver};

pub struct Cpu {
    v_registers: [u8; 16],
    i_register: u8,
    // timer: u8,
    // sound: u8,
    program_counter: u16,
    // stack_pointer: u8,
    // stack: [u16; 16],
}

impl Cpu {
    pub fn new(memory_start: u16) -> Cpu {
        Cpu {
            v_registers: [0; 16],
            i_register: 0,
            program_counter: memory_start,
        }
    }

    pub fn i_register(&self) -> u8 {
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

        println!("{:04x}", opcode);

        match opcode & 0xF000
        {
            0x0000 => {
                match opcode
                {
                    0x00E0 => {
                        // 0x00E0: Clears the screen.
                        println!("Clears the screen.");
                        display.clear();
                    },
                    _ => {
                        panic!("Unknown opcode: {:04x}", opcode);
                    }
                }
            },
            0x1000 => {
                // 0x1NNN: Jumps to address NNN.
                let address = opcode & 0x0FFF;
                self.program_counter = address;
            }
            0xA000 => {
                println!("Set the I Reg.");
                self.i_register = (opcode & 0x0FFF) as u8;
            },
            0x6000 => {
                println!("Set Vx to NN.");
                let ix = ((opcode & 0x0F00) >> 8) as usize;
                self.v_registers[ix] = (opcode & 0x00FF) as u8;
            },
            0xD000 => {
                println!("TODO: Drwa the stuff.");
                let x = self.v_registers[((opcode & 0x0F00) >> 8) as usize];
                let y = self.v_registers[((opcode & 0x00F0) >> 4) as usize];
                let n = (opcode & 0x000F) as u8;

                for i in 0..n {
                    let pixel_location = memory.read_byte(self.i_register + i);
                    let pixels = memory.read_byte(pixel_location);
                    println!("{:02x}", pixel_location);
                    println!("{:02x}", pixels);
                    display.set_pixels(x, y + i, pixels);
                }
            },
            0x7000 => { 
                println!("Add NN to Vx.");
                let ix = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v_registers[ix] = self.v_registers[ix].saturating_add(nn);
            },
            _ => {
                panic!("Unknown opcode: {:04x}", opcode);
            }
        }

        // decode
        // execute
    }

    // pub(crate) fn cycle2(&self, memory: &mut Memory, display: &mut Display<crate::sdl2_display_driver::Sdl2DisplayDriver>) -> _ {
    //     todo!()
    // }


}

// impl fmt::Debug for Memory {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ data: {:?} }}", self.data)
//     }
// }
