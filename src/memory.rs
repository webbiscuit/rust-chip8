use std::fmt;

pub struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 4096],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn write_bytes(&mut self, address: u16, data: &[u8]) {
        self.data[address as usize..(address as usize + data.len())].copy_from_slice(data);
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ data: {:?} }}", self.data)
    }
}
