use crate::{chip8::{Cpu, Instruction}, error, info};

impl Cpu {
    /// `0x2NNN/CALL` -> Call a subroutine.
    pub fn op_2(&mut self, ins: &Instruction) {
        if self.stack_pointer == 16 {
            error!("Stack Overflow");
            panic!()
        } else {
            self.stack.push(self.pc);
            self.stack_pointer += 1;
        }
        self.pc = ins.nnn;
    }
}