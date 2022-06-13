//! Contains miscellaneous functionality used in various places throughout this application.
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