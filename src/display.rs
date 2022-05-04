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
}

impl<DisplayDriverT: DisplayDriver> Display<DisplayDriverT> {
    pub fn new(display_driver: DisplayDriverT) -> Display<DisplayDriverT> {
        let mut display = Display{
            screen: [false; WIDTH * HEIGHT],
            display_driver
        };

        display.display_driver.draw_pixels(display.screen);

        display
    }
}

// impl fmt::Debug for Display<DisplayDriverT> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ screen: {:?} }}", self.screen)
//     }
// }