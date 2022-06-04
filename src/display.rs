pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub trait DisplayDriver {
    fn draw_pixels(&mut self, data: [bool; WIDTH * HEIGHT]);
}

#[derive(Debug)]
pub struct Display<DisplayDriverT: DisplayDriver> {
    screen: [bool; WIDTH * HEIGHT],
    display_driver: DisplayDriverT,
    dirty: bool,
    collision: bool
}

impl<DisplayDriverT: DisplayDriver> Display<DisplayDriverT> {
    pub fn new(display_driver: DisplayDriverT) -> Display<DisplayDriverT> {
        Display{
            screen: [false; WIDTH * HEIGHT],
            display_driver,
            dirty: true,
            collision: false
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

    pub fn did_collide(&self) -> bool {
        self.collision
    }

    pub fn set_pixels(&mut self, x: u8, y: u8, pixels: u8, start_y: u8) {
        let clip_y = (((start_y as usize / HEIGHT) + 1) * HEIGHT) as u16;

        if (y as u16) >= clip_y {
            return;
        }
        
        let ix = (x as usize % WIDTH) + ((y as usize % HEIGHT) * WIDTH);
        for i in 0..8 {
            let pixel = pixels & (1 << (7 - i)) != 0;

            // Screen width
            let clip_x = WIDTH + ((y as usize % HEIGHT) * WIDTH);

            let pixel_pos = ix + i;
            
            if pixel_pos < clip_x {
                self.collision |= self.screen[pixel_pos] & pixel;
                self.screen[pixel_pos] ^= pixel;
            }
        }

        self.dirty = true;
    }

    pub fn begin_draw(&mut self) {
        self.collision = false;
    }
}