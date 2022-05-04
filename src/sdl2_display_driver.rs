use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::display::{DisplayDriver, WIDTH, HEIGHT};

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (HEIGHT as u32) * SCALE_FACTOR;

pub struct Sdl2DisplayDriver {
    canvas: Canvas<Window>,
}

impl Sdl2DisplayDriver {
    pub fn new() -> Sdl2DisplayDriver {
        let sdl_context = sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Sdl2DisplayDriver { canvas: canvas }
    }
}

impl DisplayDriver for Sdl2DisplayDriver {
    fn draw_pixels(&mut self, pixels: [bool; WIDTH * HEIGHT]) {
        for (i, pixel_on) in pixels.iter().enumerate() {
            let x = i % WIDTH * SCALE_FACTOR as usize;
            let y = i / WIDTH * SCALE_FACTOR as usize;

            self.canvas.set_draw_color(to_colour(*pixel_on));
            let _ = self.canvas
                .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        }
        self.canvas.present();
    }
}

fn to_colour(value: bool) -> pixels::Color {
    if value == false {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 255, 102)
    }
}