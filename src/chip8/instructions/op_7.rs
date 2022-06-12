use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0x7XKK/ADD` -> Set Vx += KK
    pub fn op_7(&mut self, ins: Instruction) {
        self.v[ins.x() as usize] = self.v[ins.x() as usize].wrapping_add(ins.kk());
    }
}