use crate::chip8::{Cpu, Instruction};

impl Cpu {
    /// `0x3XKK/SE` -> Skip the next instruction if Vx == KK
    pub fn op_3(&mut self, ins: Instruction) {
        if self.v[ins.x() as usize] == ins.kk() {
            self.pc += 2;
        }
    }
}