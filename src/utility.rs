//! Contains miscellaneous functionality.
#![allow(unused)]

use crate::error;

/// Hooks panic messages.
pub fn hook_panic() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            // Explcit panics will be used to quickly exit the program.
            if *s != "explicit panic" {
                error!("An unhandled runtime exception occured: {}\n\nPlease open an issue on the Github repository (found here: https://github.com/v-maxson/Memu).", s);
            }
        } else {
            error!("An unhandled runtime exception occured, please open an issue on the Github repository (found here: https://github.com/v-maxson/Memu).");
        }
    }));
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