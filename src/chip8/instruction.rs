//! This module contains the "Instruction" struct for representing CHIP-8 instructions.

use bitfield::bitfield;

bitfield! {
    /// Represents a single CHIP-8 instruction. 
    #[derive(Copy, Clone)]
    pub struct Instruction(u16);

    u16;
    pub full, set_full: 15, 0;
    pub nnn, set_nnn: 11, 0;

    u8;
    pub op, set_op: 15, 12;
    pub x, set_x: 11, 8;
    pub y, set_y: 7, 4;
    pub n, set_n: 3, 0;
    pub jj, set_jj: 15, 8;
    pub kk, set_kk: 7, 0;
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Instruction> for u16 {
    fn from(value: Instruction) -> Self {
        value.full()
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
        write!(f, "{:#06X}", self.full())
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Full: {:#06X}, Op: {:#03X}, X: {:#02X}, Y: {:#02X}, N: {:#02X}, KK: {:#03X}, NNN: {:#05X} }}",
                self.full(), self.op(), self.x(), self.y(), self.n(), self.kk(), self.nnn())
    }
}