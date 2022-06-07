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
    map.insert(0x2, Cpu::op_2);
    map.insert(0x3, Cpu::op_3);
    map.insert(0x4, Cpu::op_4);
    map.insert(0x5, Cpu::op_5);
    map.insert(0x6, Cpu::op_6);
    map.insert(0x7, Cpu::op_7);

    map
};