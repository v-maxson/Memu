//! This file contains the CHIP-8 portion of this application.
//! This is by far the simplest "emulator" in this collection.
//! For more information on the history of the CHIP-8 and why I put emulator in quotes: 
//! 
//! https://en.wikipedia.org/wiki/CHIP-8
#![allow(unused)]

mod cpu; pub use cpu::*;
mod instruction; pub use instruction::*;
mod ins_table; pub use ins_table::*;
mod instructions; pub use instructions::*;
mod builtins; pub use builtins::*;