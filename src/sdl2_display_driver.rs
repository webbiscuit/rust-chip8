use crate::display::{DisplayDriver, WIDTH, HEIGHT};

#[derive(Debug)]
pub struct Sdl2DisplayDriver {
}

impl Sdl2DisplayDriver {
    pub fn new() -> Sdl2DisplayDriver {
        Sdl2DisplayDriver {
        }
    }
}

impl DisplayDriver for Sdl2DisplayDriver {
    fn draw_pixels(&self, data: [u8; WIDTH * HEIGHT]) {
        unimplemented!()
    }
}
