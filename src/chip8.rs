use std::fmt;

use sdl2::Sdl;

use crate::memory::Memory;
use crate::display::Display;
use crate::sdl2_display_driver::Sdl2DisplayDriver;
use crate::cpu::Cpu;
use crate::keyboard::Keyboard;

pub const PROGRAM_START: u16 = 0x200;
pub const FONT_START: u16 = 0x050;

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu,
    display: Display<Sdl2DisplayDriver>,
    keyboard: Keyboard,
}

impl Chip8 {
    pub fn new (sdl_context: &Sdl) -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            cpu: Cpu::new(PROGRAM_START),
            display: Display::new(Sdl2DisplayDriver::new(sdl_context)),
            keyboard: Keyboard::new(),
        }
    }

    pub fn load_default_font(&mut self) {
        self.memory.write_bytes(FONT_START, &[
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F   
        ]);
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory.write_bytes(PROGRAM_START, data);
    }

    pub fn precycle(&mut self) {
        self.keyboard.clear_signals();
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.memory, &mut self.display, &self.keyboard);
        self.display.draw_if_dirty();
    }

    pub fn show_internals(&self) {
        println!("Registers");

        println!("  PC: 0x{:04x}", self.cpu.program_counter());
        println!("   I: 0x{:02x}", self.cpu.i_register());

        for i in 0..16 {
            println!("  V{:1X}: 0x{:02x}", i, self.cpu.v_registers()[i]);
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn timer_cycle(&mut self) {
        self.cpu.timer_cycle();
    }

    pub fn key_down(&mut self, key: u8) {
        self.keyboard.key_down(key);
    }

    pub fn key_up(&mut self) {
        self.keyboard.key_up();
    }

    
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ memory: {:?} }}", self.memory)
    }
}
