
pub struct Keyboard {
    key_pressed: Option<u8>,
    key_last_pressed: Option<u8>,
    signal_key_pressed: bool,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_pressed: None,
            key_last_pressed: None,
            signal_key_pressed: false,
        }
    }

    pub fn clear_signals(&mut self) {
        self.signal_key_pressed = false;
    }

    pub fn key_down(&mut self, key: u8) {
        self.key_pressed = Some(key);
    }

    pub fn key_up(&mut self) {
        self.signal_key_pressed = true;
        self.key_last_pressed = self.key_pressed;
        self.key_pressed = None
    }

    pub fn key_last_pressed(&self) -> Option<u8> {
        self.key_last_pressed
    }

    pub fn has_signal_keypress(&self) -> bool {
        self.signal_key_pressed
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.key_pressed == Some(key)
    }
}