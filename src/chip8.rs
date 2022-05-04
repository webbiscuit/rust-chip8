use std::fmt;

use crate::memory::Memory;
use crate::display::Display;
use crate::sdl2_display_driver::Sdl2DisplayDriver;

pub const PROGRAM_START: u16 = 0x200;

pub struct Chip8 {
    memory: Memory,
    display: Display<Sdl2DisplayDriver>,
}

impl Chip8 {
    pub fn new () -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            display: Display::new(Sdl2DisplayDriver::new()),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory.write_bytes(PROGRAM_START, data);
    }
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ memory: {:?} }}", self.memory)
    }
}
