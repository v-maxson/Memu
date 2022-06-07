use crate::chip8::{Cpu, Instruction, DISPLAY_DIMENSIONS, PIXEL_OFF};

impl Cpu {
    /// Contains 2 instructions:
    /// - 0x00E0/CLR -> clear the display.
    /// - 0x00EE/RET -> return from a subroutine.
    pub fn op_0(&mut self, ins: Instruction) {
        if ins.x == 0 {
            match ins.kk {
                // CLR
                0x0E => {
                    self.display_memory = [PIXEL_OFF; DISPLAY_DIMENSIONS]
                }

                // RET
                0xEE => {
                    if self.stack.pop().is_none() { panic!("Stack Underflow") }
                }

                _ => ()
            }
        }
    }
}
