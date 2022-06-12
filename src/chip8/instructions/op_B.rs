use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0xBNNN/JP` -> Jump to address NNN + V0
    pub fn op_B(&mut self, ins: Instruction) {
        self.pc = ins.nnn() + self.v[0x0] as u16;
    }
}