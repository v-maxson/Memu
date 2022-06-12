use crate::chip8::{Cpu, Instruction};
use rand::{thread_rng, Rng};

impl Cpu {
    /// `0xCXKK/RND` -> Set Vx == RANDOM AND KK
    pub fn op_C(&mut self, ins: Instruction) {
        let byte = thread_rng().gen::<u8>();
        self.v[ins.x() as usize] = byte & ins.kk();
    }
}