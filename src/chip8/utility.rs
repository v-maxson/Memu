//! Contains miscellaneous functionality used in the CHIP-8 portion of this application.
#![allow(unused)]

/// Returns the value of the bit at n.
pub fn get_bit_at(input: u8, n: u8) -> u8 {
    if n < 8 {
        input & (1 << n)
    } else {
        0
    }
}

/// Returns a array of each bit.
pub fn get_bit_arr_u8(number: u8) -> [bool; 8] {
    let mut bitarr = [false; 8];

    for (i, bit) in bitarr.iter_mut().enumerate() {
        *bit = number & (0x80 >> i) as u8 > 0;
    }
    
    bitarr
}

/// Returns a tuple for each RGBA value. 
pub fn get_rgba(value: u32) -> (u8, u8, u8, u8) {
    (
        ((value & 0xFF000000) >> 24) as u8,
        ((value & 0x00FF0000) >> 16) as u8,
        ((value & 0x0000FF00) >> 8) as u8,
        (value & 0x000000FF) as u8
    )
}