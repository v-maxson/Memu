//! This module contains built-in resources for the CHIP-8 interpreter.
#![allow(non_camel_case_types)]

use static_init::dynamic;
use rustc_hash::FxHashMap;

type BuiltinsMap = FxHashMap<&'static str, &'static [u8]>;

#[dynamic(lazy)]
pub static BUILTINS: BuiltinsMap = {
    let mut map = BuiltinsMap::default();

    map.insert("ibm", include_bytes!("../../resources/chip8/IBM.ch8"));
    map.insert("test", include_bytes!("../../resources/chip8/test.ch8"));

    map
};
