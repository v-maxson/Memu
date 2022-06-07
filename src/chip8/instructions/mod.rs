//! This module contains implementations for each CHIP-8 instruction.
//! 
//! Note: this didn't have to be seperated into multiple files, and was only done
//! for organizational purposes.

mod op_0; pub use op_0::*;
mod op_1; pub use op_1::*;
mod op_2; pub use op_2::*;
mod op_3; pub use op_3::*;
mod op_4; pub use op_4::*;
mod op_5; pub use op_5::*;