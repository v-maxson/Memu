use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0x9XY0/SNE` -> Skip the next instruction if Vx != Vy
    pub fn op_9(&mut self, ins: Instruction) {
        if self.v[ins.x() as usize] != self.v[ins.y() as usize] {
            self.pc += 2;
        }
    }
}