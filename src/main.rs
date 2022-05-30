use std::{fs::File, io::Read};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use spin_sleep::LoopHelper;

use chip8::Chip8;

mod chip8;
mod memory;
mod display;
mod sdl2_display_driver;
mod cpu;


fn main() {
    println!("Chip-8 By Dan!");

    let mut file = File::open("roms/VERS").unwrap();
    // let mut file = File::open("roms/bc_test.ch8").unwrap();
    // let mut file = File::open("roms/test_opcode.ch8").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    // print!("Data: {:02x?}", data);

    let sdl_context = sdl2::init().unwrap();
    let mut chip8 = Chip8::new(&sdl_context);
    chip8.load_default_font();
    chip8.load_rom(&data);

    // print!("{:?}", chip8);

    let mut event_pump = sdl_context.event_pump().unwrap();
    const MAX_INSTRUCTIONS_PER_SECOND: i32 = 700;
    // const MAX_INSTRUCTIONS_PER_SECOND: i32 = 100500;
    const TIMER_FREQUENCY_PER_SECOND: f64 = 60.0;

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(1.0 / TIMER_FREQUENCY_PER_SECOND)
        .build_with_target_rate(MAX_INSTRUCTIONS_PER_SECOND);


    'running: loop {
        loop_helper.loop_start();

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
                    scancode,
                    ..
                } => handle_keydown(scancode, &mut chip8),
                Event::KeyUp { 
                    ..
                } => chip8.key_up(),
                // Event::KeyDown {
                //     keycode: Some(Keycode::Space),
                //     ..
                // } => {
                //     chip8.cycle();
                //     chip8.show_internals();
                //     last_instruction_run_time = Instant::now();
                // },
                _ => {}
            }
        }

        chip8.cycle();
        chip8.show_internals();

        if let Some(_fps) = loop_helper.report_rate() {
            chip8.timer_cycle();
        }

        loop_helper.loop_sleep(); 
    }
}

fn handle_keydown(scancode: Option<Scancode>, chip8: &mut Chip8) {
    match scancode {
        Some(Scancode::Num1) => {
            chip8.key_pressed(0x00);
        },
        Some(Scancode::Num2) => {
            chip8.key_pressed(0x01);
        },
        Some(Scancode::Num3) => {
            chip8.key_pressed(0x02);
        },
        Some(Scancode::Num4) => {
            chip8.key_pressed(0x03);
        },
        Some(Scancode::Q) => {
            chip8.key_pressed(0x04);
        },
        Some(Scancode::W) => {
            chip8.key_pressed(0x05);
        },
        Some(Scancode::E) => {
            chip8.key_pressed(0x06);
        },
        Some(Scancode::R) => {
            chip8.key_pressed(0x07);
        },
        Some(Scancode::A) => {
            chip8.key_pressed(0x08);
        },
        Some(Scancode::S) => {
            chip8.key_pressed(0x09);
        },
        Some(Scancode::D) => {
            chip8.key_pressed(0x0A);
        },
        Some(Scancode::F) => {
            chip8.key_pressed(0x0B);
        },
        Some(Scancode::Z) => {
            chip8.key_pressed(0x0C);
        },
        Some(Scancode::X) => {
            chip8.key_pressed(0x0D);
        },
        Some(Scancode::C) => {
            chip8.key_pressed(0x0E);
        },
        Some(Scancode::V) => {
            chip8.key_pressed(0x0F);
        },
        _ => {}
    }
}
