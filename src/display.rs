pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub trait DisplayDriver {
    // fn new() -> Self;
    fn draw_pixels(&mut self, data: [bool; WIDTH * HEIGHT]);
}

#[derive(Debug)]
pub struct Display<DisplayDriverT: DisplayDriver> {
    screen: [bool; WIDTH * HEIGHT],
    display_driver: DisplayDriverT,
    dirty: bool
}

impl<DisplayDriverT: DisplayDriver> Display<DisplayDriverT> {
    pub fn new(display_driver: DisplayDriverT) -> Display<DisplayDriverT> {
        Display{
            screen: [false; WIDTH * HEIGHT],
            display_driver,
            dirty: true
        }
    }

    pub fn clear(&mut self) {
        self.screen = [false; WIDTH * HEIGHT];
        self.dirty = true;
    }

    pub fn draw_if_dirty(&mut self) {
        if self.dirty {
            self.display_driver.draw_pixels(self.screen);
            self.dirty = false;
        }
    }

    pub fn set_pixels(&mut self, x: u8, y: u8, pixels: u8) {
        let ix = (x as usize) + (y as usize * WIDTH);
        for i in 0..8 {
            let pixel = pixels & (1 << (7 - i)) != 0;
            let collision = self.screen[ix + i] & pixel;
            self.screen[ix + i] ^= pixel;
        }

        self.dirty = true;
    }
}

// impl fmt::Debug for Display<DisplayDriverT> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ screen: {:?} }}", self.screen)
//     }
// }