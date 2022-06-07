use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// 0x1NNN/JMP -> Jump to address NNN.
    pub fn op_1(&mut self, ins: Instruction) {
        self.pc = ins.nnn;
    }
}