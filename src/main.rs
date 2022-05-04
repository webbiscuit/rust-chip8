use std::{fs::File, io::Read};
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
}
