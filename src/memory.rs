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

    pub fn read_word(&self, program_counter: u16) -> u16 {
        let opcode = (self.data[program_counter as usize] as u16) << 8 | self.data[program_counter as usize + 1] as u16;
        opcode
    }

    pub fn read_byte(&self, i: u8) -> u8 {
        self.data[i as usize]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ data: {:02x?} }}", self.data)
    }
}
