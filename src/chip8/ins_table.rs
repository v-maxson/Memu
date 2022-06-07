//! This module contains the instruction table used by the CPU.
#![allow(non_camel_case_types)]

use super::{Cpu, Instruction};
use static_init::dynamic;
use rustc_hash::FxHashMap;

type CpuInstruction = fn(&mut Cpu, Instruction) -> ();
type InstructionTable = FxHashMap<u8, CpuInstruction>;

#[dynamic]
pub static INSTRUCTION_TABLE: InstructionTable = {
    let mut map: InstructionTable = InstructionTable::default();

    map.insert(0x0, Cpu::op_0);
    map.insert(0x1, Cpu::op_1);

    map
};