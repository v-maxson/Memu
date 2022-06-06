use crate::chip8::{ Cpu, Instruction };

impl Cpu {
    /// Contains 2 instructions:
    /// 0x00EE/RET -> return from a subroutine.
    /// 0x00E0/CLR -> clear the display.
    pub fn op_0(&mut self, ins: Instruction) {
        if ins.x == 0 {
            
        }
    }

}
