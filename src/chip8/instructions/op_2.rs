use crate::{chip8::{Cpu, Instruction}, error};

impl Cpu {
    /// 0x2NNN/CALL -> Call a subroutine.
    pub fn op_2(&mut self, ins: Instruction) {
        if self.stack.len() == 16 {
            error!("Stack Overflow");
            panic!()
        } else {
            self.stack.push(self.pc);
        }
        self.pc = ins.nnn;
    }
}