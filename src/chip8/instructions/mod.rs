//! This module contains implementations for each CHIP-8 instruction.
#![allow(non_snake_case)]

mod op_0; pub use op_0::*;
mod op_1; pub use op_1::*;
mod op_2; pub use op_2::*;
mod op_3; pub use op_3::*;
mod op_4; pub use op_4::*;
mod op_5; pub use op_5::*;
mod op_6; pub use op_6::*;
mod op_7; pub use op_7::*;
mod op_8; pub use op_8::*;
mod op_9; pub use op_9::*;
mod op_A; pub use op_A::*;
mod op_B; pub use op_B::*;
mod op_C; pub use op_C::*;
mod op_D; pub use op_D::*;