//! This module contains the "Instruction" struct for representing CHIP-8 instructions.

use bitfield::bitfield;

bitfield! {
    #[derive(Copy, Clone)]
    struct __OPXYN(u16);
    n, _: 3, 0;
    y, _: 7, 4;
    x, _: 11, 8;
    op, _: 15, 12;
}

bitfield! {
    #[derive(Copy, Clone)]
    struct __JJKK(u16);
    kk, set_kk: 7, 0;
    jj, set_jj: 15, 8;
}

bitfield! {
    #[derive(Copy, Clone)]
    struct __NNN(u16);
    nnn, _: 11, 0;
}

/// Represents a single CHIP-8 instruction. 
pub union Instruction {
    full: u16,
    opxyn: __OPXYN,
    jjkk: __JJKK,
    nnn: __NNN
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self { full: value }
    }
}

impl From<Instruction> for u16 {
    fn from(value: Instruction) -> Self {
        value.full()
    }
}

impl Instruction {
    pub fn from_u8_pair(high: u8, low: u8) -> Self {
        unsafe {
            let mut new = Self { full: 0x0000 };
            new.jjkk.set_jj(high as u16);
            new.jjkk.set_kk(low as u16);
            new
        }
    }

    pub fn full(&self) -> u16 {
        unsafe {self.full}
    }

    #[inline(always)]
    pub fn op(&self) -> u8 {
        unsafe {self.opxyn.op() as u8}
    }

    #[inline(always)]
    pub fn x(&self) -> u8 {
        unsafe {self.opxyn.x() as u8}
    }

    #[inline(always)]
    pub fn y(&self) -> u8 {
        unsafe {self.opxyn.y() as u8}
    }

    #[inline(always)]
    pub fn n(&self) -> u8 {
        unsafe {self.opxyn.n() as u8}
    }

    #[inline(always)]
    pub fn jj(&self) -> u8 {
        unsafe {self.jjkk.jj() as u8}
    }

    #[inline(always)]
    pub fn kk(&self) -> u8 {
        unsafe {self.jjkk.kk() as u8}
    }

    #[inline(always)]
    pub fn nnn(&self) -> u16 {
        unsafe {self.nnn.nnn()}
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