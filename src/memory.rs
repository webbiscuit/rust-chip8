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

    pub fn read_word(&self, address: u16) -> u16 {
        let opcode = (self.data[address as usize] as u16) << 8 | self.data[address as usize + 1] as u16;
        opcode
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ data: {:02x?} }}", self.data)
    }
}
