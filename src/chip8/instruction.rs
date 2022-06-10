//! This module contains the "Instruction" struct for representing CHIP-8 instructions.

/// Represents a single CHIP-8 instruction. 
pub struct Instruction {
    /// The full instruction.
    pub full: u16,

    /// The first nibble of the instruction, used to determine what instruction is being called.
    pub op: u8,

    /// The second nibble of the instruction.
    pub x: u8,

    /// The third nibble of the instruction.
    pub y: u8,

    /// The fourth nibble of the instruction.
    pub n: u8,

    /// The last byte of the instruction.
    pub kk: u8,

    /// The last 12 bits of the instruction.
    pub nnn: u16
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Full: {:#06X}, Opcode: {:#03X}, X: {:#03X}, Y: {:#03X}, N: {:#03X}, KK: {:#04X}, NNN: {:#05X} }}",
                self.full, self.op, self.x, self.y, self.n, self.kk, self.nnn)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.full)
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let full = value;
        let op = ((full & 0xF000) >> 12) as u8;
        let x = ((full & 0x0F00) >> 8) as u8;
        let y = ((full & 0x00F0) >> 4) as u8;
        let n = (full & 0x000F) as u8;
        let kk = (full & 0x00FF) as u8;
        let nnn = (full & 0x0FFF) as u16;

        Self { full, op, x, y, n, kk, nnn }
    }
}

impl Instruction {
    pub fn from_u8_pair(high: u8, low: u8) -> Self {
        let value: u16 = ((high as u16) << 8) | (low as u16);
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn instruction_test() {
        let ins: Instruction = Instruction::from_u8_pair(0xAB, 0xCD);
        println!("Display: {}, Debug: {:?}", ins, ins);
    }

}
