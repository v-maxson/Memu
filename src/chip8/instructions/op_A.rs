use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0xANNN/LD` -> Set I = NNN
    pub fn op_A(&mut self, ins: Instruction) {
        self.i = ins.nnn;
    }
}