use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0x6XKK/LD` -> Set Vx = KK
    pub fn op_6(&mut self, ins: Instruction) {
        self.v[ins.x as usize] = ins.kk;
    }
}