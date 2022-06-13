use winit::event::VirtualKeyCode;
use crate::chip8::{Cpu, Instruction, convert_to_key};

impl Cpu {
    /// Contains 2 instructions:
    /// - `0xEX9E/SKP` -> Skip the next instruction if the key stored in Vx is pressed.
    /// - `0xEXA1/SKNP` -> Skip the next instruction if the key stored in Vx is NOT pressed.
    pub fn op_E(&mut self, ins: Instruction) {
        match ins.kk() {
            0x9E => {
                if self.input.key_pressed(convert_to_key(ins.x()).unwrap()) {
                    self.pc += 2;
                }
            }

            0xA1 => {
                if !self.input.key_pressed(convert_to_key(ins.x()).unwrap()) {
                    self.pc += 2;
                }
            }

            _ => ()
        }
    }
}