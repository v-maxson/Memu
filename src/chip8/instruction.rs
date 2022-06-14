//! This module contains the "Instruction" struct for representing CHIP-8 instructions.

use bitfield::bitfield;

bitfield! {
    /// Represents a single CHIP-8 instruction. 
    #[derive(Copy, Clone)]
    pub struct Instruction(u16);

    u16;

    /// Gets/Sets the full instruction value.
    pub get, set: 15, 0;

    /// Gets the last 12 bits of the instruction.
    pub nnn, _: 11, 0;

    u8;

    /// Gets the first nibble of the instruction.
    pub op, _: 15, 12;

    /// Gets the second nibble of the instruction.
    pub x, _: 11, 8;

    /// Gets the third nibble of the instruction.
    pub y, _: 7, 4;

    /// Gets the fourth nibble of the instruction.
    pub n, _: 3, 0;

    /// Gets the last byte of the instruction.
    pub kk, _: 7, 0;

    _, set_jj: 15, 8;
    _, set_kk: 7, 0;
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Instruction> for u16 {
    fn from(value: Instruction) -> Self {
        value.get()
    }
}

impl Instruction {
    pub fn from_u8_pair(high: u8, low: u8) -> Self {
        let mut new = Self(0x0000);
        new.set_jj(high);
        new.set_kk(low);
        new
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", self.get())
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Full: {:#06X}, Op: {:#03X}, X: {:#02X}, Y: {:#02X}, N: {:#02X}, KK: {:#03X}, NNN: {:#05X} }}",
                self.get(), self.op(), self.x(), self.y(), self.n(), self.kk(), self.nnn())
    }
}