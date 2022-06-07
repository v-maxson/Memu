use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0x4XKK/SNE` -> Skip the next instruction if Vx != KK
    pub fn op_4(&mut self, ins: Instruction) {
        if self.v[ins.x as usize] != ins.kk {
            self.pc += 2;
        }
    }
}