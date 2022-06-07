//! This module contains the CPU for the CHIP-8.

use smallvec::{SmallVec, smallvec};

pub const MEMORY_SIZE: usize = 0x1000;
pub const V_REG_COUNT: usize = 0x10;
pub const STACK_SIZE: usize = 0x10;
pub const PC_START: usize = 0x200;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_DIMENSIONS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
pub const PIXEL_ON: bool = true;
pub const PIXEL_OFF: bool = false;

pub struct Cpu {
    /// The CHIP-8 had 4KB of memory available to programs.
    /// 
    /// The original CHIP-8 interpreter was stored (in its entirety) 
    /// in the first 512 bytes of this memory space. In this implemenation,
    /// only the base fontset will be stored in this memory, and the rest
    /// will simply be empty. 
    pub memory: [u8; MEMORY_SIZE],

    /// The CHIP-8 had a 64x32 monochrome display. 
    pub display_memory: [bool; DISPLAY_DIMENSIONS],

    /// The CHIP-8 had 16 general-purpose 8-bit registers collectively called V0 through Vf.
    /// 
    /// Note: Register Vf is used internally by some instructions. While programs can still
    /// modify the value in this register directly, it is unwise to do so as the value may change
    /// depending on the outcome of instructions that follow.
    pub v: [u8; V_REG_COUNT],

    /// One of 2 timers. 
    /// 
    /// The ST or Sound Timer makes a beep while it's value is above 0.
    pub st: u8,

    /// One of 2 timers.
    /// 
    /// The DT or Delay Timer is generally used to delay execution of something,
    /// but can be used for anything that requirs timing.
    pub dt: u8,

    /// The PC or Program Counter (sometimes referred to as the "Instruction Pointer")
    /// points to the next instruction in memory to be excecuted.
    pub pc: u16,

    /// The I or Index register is used to store addresses in memory.
    /// 
    /// This is generally used by the draw instruction to point to sprites
    /// to be drawn.
    pub i: u16,

    /// The Stack is a basic stack data-structure that is used to store
    /// the value of the PC to return from subroutines.
    pub stack: SmallVec<[u16; STACK_SIZE]>
}

impl Cpu {
    // This isn't implemented via the Default trait so as to keep it private.
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            display_memory: [false; DISPLAY_DIMENSIONS],
            v: [0; V_REG_COUNT],
            st: 0,
            dt: 0,
            pc: PC_START as u16,
            i: 0,
            stack: smallvec![0; STACK_SIZE]
        }
    }

    pub fn start(rom_path: &str, clock_speed_hz: u64) {
        let mut cpu = Cpu::default();
    }
}
