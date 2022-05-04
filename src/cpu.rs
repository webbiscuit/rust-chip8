use std::fmt;

pub struct Cpu {
    v_registers: [u8; 16],
    i_register: u8,
    timer: u8,
    sound: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Register {
        Register {
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
