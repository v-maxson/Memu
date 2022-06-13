use crate::chip8::{Cpu, Instruction, DISPLAY_WIDTH, DISPLAY_HEIGHT, PIXEL_ON, PIXEL_OFF};

/// Returns a array of each bit.
pub fn get_bit_arr_u8(number: u8) -> [bool; 8] {
    let mut bitarr = [false; 8];

    for (i, bit) in bitarr.iter_mut().enumerate() {
        *bit = number & (0x80 >> i) as u8 > 0;
    }
    
    bitarr
}

impl Cpu {
    /// `0xDXYN/DRW` -> Draw an N-byte sprite starting at I in memory.
    pub fn op_D(&mut self, ins: Instruction) {
        let x = self.v[ins.x() as usize] % DISPLAY_WIDTH as u8;
        let y = self.v[ins.y() as usize] % DISPLAY_HEIGHT as u8;

        // Set VF to 0.
        self.v[0xF] = 0;

        for row in 0..ins.n() {
            // Get the nth byte of sprite data pointed to by I.
            let sprite_byte = self.memory[self.i as usize + row as usize];

            for (col, bit) in get_bit_arr_u8(sprite_byte).iter().enumerate() {
                let screen_pixel = &mut self.display_memory[(y as usize + row as usize) * DISPLAY_WIDTH + (x as usize + col as usize)];

                if *bit {

                    if *screen_pixel == PIXEL_ON { 
                        self.v[0xF] = 1;
                        *screen_pixel = PIXEL_OFF;
                    } else {
                        *screen_pixel = PIXEL_ON;
                    }
                }
            }
        }
    }
}