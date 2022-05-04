use std::{fs::File, io::Read};
use std::io;
use std::io::prelude::*;

use chip8::Chip8;

mod chip8;
mod memory;
mod display;
mod sdl2_display_driver;

fn main() {
    println!("Chip-8 By Dan!");

    let mut file = File::open("roms/IBM Logo.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    // print!("Data: {:?}", data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    print!("{:?}", chip8);

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
