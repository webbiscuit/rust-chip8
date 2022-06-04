use crate::cpu::{Address, Register, Value};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen, // 00E0 
    Return, // 00EE 
    Jump(Address), // 1NNN
    Call(Address), // 2NNN
    CheckEqualValue(Register, Value), // 3XNN
    CheckNotEqualValue(Register, Value), // 4XNN
    CheckEqual(Register, Register), // 5XY0
    SetRegisterToValue(Register, Value), // 6XNN
    AddValueToRegister(Register, Value), // 7XNN
    SetRegister(Register, Register), // 8XY0
    Or(Register, Register), // 8XY1
    And(Register, Register), // 8XY2
    Xor(Register, Register), // 8XY3
    Add(Register, Register), // 8XY4
    Subtract{ destination: Register, first: Register, second: Register }, // 8XY5/8XY7
    ShiftRight(Register), // 8XY6
    ShiftLeft(Register), // 8XYE
    CheckNotEqual(Register, Register), // 9XY0
    SetIndex(Address), // ANNN
    // BNNN Jump with offset
    Random(Register, Value), // CXNN
    Display{ vx: Register, vy: Register, pixel_height: Value }, // DXYN
    SkipIfKeyPressed(Register), // EX9E
    SkipIfKeyNotPressed(Register), // EXA1
    ReadDelayTimer(Register), // FX07
    WriteDelayTimer(Register), // FX15
    WriteSoundTimer(Register), // FX18
    AddRegisterToIndex(Register), // FX1E
    WaitForKeyPress(Register), // FX0A
    SetIndexToSprite(Register), // FX29
    StoreBCD(Register), // FX33
    StoreRegisters(Register), // FX55
    LoadRegisters(Register), // FX65
} 

impl Instruction {
    pub fn decode(opcode: u16) -> Instruction {
        let nibble1 = ((opcode & 0xF000) >> 12) as u8;
        let nibble2 = ((opcode & 0x0F00) >> 8) as u8;
        let nibble3 = ((opcode & 0x00F0) >> 4) as u8;
        let nibble4 = (opcode & 0x000F) as u8; 

        let instruction = match(nibble1, nibble2, nibble3, nibble4)
        {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
            (0x1, _, _, _) => Instruction::Jump(opcode & 0x0FFF),
            (0x2, _, _, _) => Instruction::Call(opcode & 0x0FFF),
            (0x3, x, _, _) => Instruction::CheckEqualValue(x, (opcode & 0x00FF) as u8),
            (0x4, x, _, _) => Instruction::CheckNotEqualValue(x, (opcode & 0x00FF) as u8),
            (0x5, x, y, 0) => Instruction::CheckEqual(x, y),
            (0x6, x, _, _) => Instruction::SetRegisterToValue(x, (opcode & 0x00FF) as u8),
            (0x7, x, _, _) => Instruction::AddValueToRegister(x, (opcode & 0x00FF) as u8),
            (0x8, x, y, 0) => Instruction::SetRegister(x, y),
            (0x8, x, y, 1) => Instruction::Or(x, y),
            (0x8, x, y, 2) => Instruction::And(x, y),
            (0x8, x, y, 3) => Instruction::Xor(x, y),
            (0x8, x, y, 4) => Instruction::Add(x, y),
            (0x8, x, y, 5) => Instruction::Subtract{ destination: x, first: x, second: y},
            (0x8, x, _, 6) => Instruction::ShiftRight(x),
            (0x8, x, y, 7) => Instruction::Subtract{ destination: x, first: y, second: x},
            (0x8, x, _, 0xE) => Instruction::ShiftLeft(x),
            (0x9, x, y, 0) => Instruction::CheckNotEqual(x, y),
            (0xA, _, _, _) => Instruction::SetIndex(opcode & 0x0FFF),
            (0xC, x, _, _) => Instruction::Random(x, (opcode & 0x00FF) as u8),
            (0xD, x, y, n) => Instruction::Display{ vx: x, vy: y, pixel_height: n },
            (0xE, x, 0x9, 0xE) => Instruction::SkipIfKeyPressed(x),
            (0xE, x, 0xA, 0x1) => Instruction::SkipIfKeyNotPressed(x),
            (0xF, x, 0x0, 0x7) => Instruction::ReadDelayTimer(x),
            (0xF, x, 0x1, 0x5) => Instruction::WriteDelayTimer(x),
            (0xF, x, 0x1, 0x8) => Instruction::WriteSoundTimer(x),
            (0xF, x, 0x1, 0xE) => Instruction::AddRegisterToIndex(x),
            (0xF, x, 0x0, 0xA) => Instruction::WaitForKeyPress(x),
            (0xF, x, 0x2, 0x9) => Instruction::SetIndexToSprite(x),
            (0xF, x, 0x3, 0x3) => Instruction::StoreBCD(x),
            (0xF, x, 0x5, 0x5) => Instruction::StoreRegisters(x),
            (0xF, x, 0x6, 0x5) => Instruction::LoadRegisters(x),
            _ => panic!("Unknown opcode: {:#06X}", opcode)
        };

        instruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_clear_screen() {
        let opcode = 0x00E0;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::ClearScreen);
    }

    #[test]
    fn test_decode_jump() {
        let opcode = 0x1ABC;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::Jump(0xABC));
    }

    #[test]
    fn test_decode_set_register() {
        let opcode = 0x6ABC;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::SetRegisterToValue(0xA, 0xBC));
    }

    #[test]
    fn test_decode_add_value() {
        let opcode = 0x7ABC;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::AddValueToRegister(0xA, 0xBC));
    }

    #[test]
    fn test_decode_set_index() {
        let opcode = 0xAABC;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::SetIndex(0xABC));
    }

    #[test]
    fn test_decode_display() {
        let opcode = 0xD123;
        let instruction = Instruction::decode(opcode);
        assert_eq!(instruction, Instruction::Display{vx: 1, vy: 2, pixel_height: 3});
    }
}