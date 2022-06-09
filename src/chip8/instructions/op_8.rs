use crate::{chip8::{Cpu, Instruction}, utility::get_bit_at};

impl Cpu {
    /// Contains 8 instructions:
    /// - `0x8XY0/LD`     -> Set Vx = Vy
    /// - `0x8XY1/OR`     -> Set Vx |= Vy
    /// - `0x8XY2/AND`    -> Set Vx &= Vy
    /// - `0x8XY3/XOR`    -> Set Vx ^= Vy
    /// - `0x8XY4/ADD`    -> Set Vx += Vy, set Vf to overflow.
    /// - `0x8XY5/SUB`    -> Set Vx -= Vy, set Vf to underflow.
    /// - `0x8XY6/SHR`    -> Set Vx >>= 1, set Vf to shifted bit.
    /// - `0x8XY7/SUBN`   -> Set Vx = Vy - Vx, set Vf to underflow.
    /// - `0x8XYE/SHL`    -> Set Vx <<= 1, set Vf to shifted bit.
    pub fn op_8(&mut self, ins: &Instruction) {
        match ins.n {
            0x0 => {
                self.v[ins.x as usize] = self.v[ins.y as usize];
            }

            0x1 => {
                self.v[ins.x as usize] |= self.v[ins.y as usize];
            }

            0x2 => {
                self.v[ins.x as usize] &= self.v[ins.y as usize];
            }

            0x3 => {
                self.v[ins.x as usize] ^= self.v[ins.y as usize];
            }

            0x4 => {
                let (result, overflow_occured) = self.v[ins.x as usize].overflowing_add(self.v[ins.y as usize]);
                if overflow_occured { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[ins.x as usize] = result;
            }

            0x5 => {
                let (result, underflow_occured) = self.v[ins.x as usize].overflowing_sub(self.v[ins.y as usize]);
                if underflow_occured { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[ins.x as usize] = result;
            }

            0x6 => {
                self.v[0xF] = get_bit_at(self.v[ins.x as usize], 7);
                self.v[ins.x as usize] >>= 1;
            }

            0x7 => {
                let (result, underflow_occured) = self.v[ins.y as usize].overflowing_sub(self.v[ins.x as usize]);
                if underflow_occured { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[ins.x as usize] = result;
            }

            0xE => {
                self.v[0xF] = get_bit_at(self.v[ins.x as usize], 0);
                self.v[ins.x as usize] <<= 1;
            }

            _ => ()
        }
    }
}