use std::time::{Instant, Duration};
use std::{fs::File, io::Read};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use chip8::Chip8;

mod chip8;
mod memory;
mod display;
mod sdl2_display_driver;
mod cpu;


fn main() {
    println!("Chip-8 By Dan!");

    // let mut file = File::open("roms/bc_test.ch8").unwrap();
    let mut file = File::open("roms/test_opcode.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    // print!("Data: {:02x?}", data);

    let sdl_context = sdl2::init().unwrap();
    let mut chip8 = Chip8::new(&sdl_context);
    chip8.load_rom(&data);

    // print!("{:?}", chip8);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: u32 = 0;

    let mut last_instruction_run_time = Instant::now();
    let mut intruction_count = 0;
    let Max_Instructions_Per_Second = 700;


    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            // print!("{:?}", event);
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    chip8.cycle();
                    chip8.show_internals();
                    last_instruction_run_time = Instant::now();
                },
                _ => {}
            }
        }

        // TODO: cap cpu speed
        // let now = Instant::now();
        // let sleep_dur = frame_duration
        //     .checked_sub(now.saturating_duration_since(timestamp))
        //     .unwrap_or(Duration::new(0, 0));
        // ::std::thread::sleep(sleep_dur);
        // timestamp = now;
        // chip8.cycle();

        chip8.cycle();
        chip8.show_internals();
        last_instruction_run_time = Instant::now();
    }
}
