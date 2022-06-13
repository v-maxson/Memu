//! Contains miscellaneous functionality used in the CHIP-8 portion of this application.
#![allow(unused)]

use winit::event::VirtualKeyCode;

pub const ALLOWED_KEYS: [VirtualKeyCode; 16] = [
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Key4,
    VirtualKeyCode::Q,
    VirtualKeyCode::W,
    VirtualKeyCode::E,
    VirtualKeyCode::R,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::F,
    VirtualKeyCode::Z,
    VirtualKeyCode::X,
    VirtualKeyCode::C,
    VirtualKeyCode::V
];

pub fn convert_to_key(value: u8) -> Option<VirtualKeyCode> {
    match value {
        0x0 => Some(VirtualKeyCode::Key1),
        0x1 => Some(VirtualKeyCode::Key2),
        0x2 => Some(VirtualKeyCode::Key3),
        0x3 => Some(VirtualKeyCode::Key4),
        0x4 => Some(VirtualKeyCode::Q),
        0x5 => Some(VirtualKeyCode::W),
        0x6 => Some(VirtualKeyCode::E),
        0x7 => Some(VirtualKeyCode::R),
        0x8 => Some(VirtualKeyCode::A),
        0x9 => Some(VirtualKeyCode::S),
        0xA => Some(VirtualKeyCode::D),
        0xB => Some(VirtualKeyCode::F),
        0xC => Some(VirtualKeyCode::Z),
        0xD => Some(VirtualKeyCode::X),
        0xE => Some(VirtualKeyCode::C),
        0xF => Some(VirtualKeyCode::V),

        _ => None
    }
}

pub fn convert_key(value: VirtualKeyCode) -> Option<u8> {
    match value {
        VirtualKeyCode::Key1 => Some(0x0),
        VirtualKeyCode::Key2 => Some(0x1),
        VirtualKeyCode::Key3 => Some(0x2),
        VirtualKeyCode::Key4 => Some(0x3),
        VirtualKeyCode::Q => Some(0x4),
        VirtualKeyCode::W => Some(0x5),
        VirtualKeyCode::E => Some(0x6),
        VirtualKeyCode::R => Some(0x7),
        VirtualKeyCode::A => Some(0x8),
        VirtualKeyCode::S => Some(0x9),
        VirtualKeyCode::D => Some(0xA),
        VirtualKeyCode::F => Some(0xB),
        VirtualKeyCode::Z => Some(0xC),
        VirtualKeyCode::X => Some(0xD),
        VirtualKeyCode::C => Some(0xE),
        VirtualKeyCode::V => Some(0xF),

        _ => None
    }
}

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