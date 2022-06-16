//! Contains the logic for the CLI.
#![allow(unused, unreachable_patterns, unstable_name_collisions)]

use clap::{Parser, Subcommand};
use crate::{error, chip8::{self, BUILTINS}, info};
use itertools::Itertools;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArguments {
    #[clap(subcommand)]
    command: Command
}

#[derive(Subcommand)]
#[non_exhaustive]
enum Command {
    /// The CHIP-8 emulator.
    Chip {
        /// The ROM to execute.
        #[clap(short, long, default_value = "")]
        rom: String,

        /// The CPU speed (in hz) 
        #[clap(long = "speed", default_value_t = 500)]
        clock_speed: u16,

        /// The scale of the display.
        #[clap(short = 's', long = "scale", default_value_t = 16)]
        display_scale: u8,

        /// Prints a list of built-in ROMs and exits.
        #[clap(short)]
        list: bool
    }
}

/// Parses CLI arguments for this program.
pub fn run() {
    let args = CliArguments::parse();

    match args.command {
        Command::Chip { rom, clock_speed, display_scale, list } => {
            if list {
                let builtin_list: String = BUILTINS.iter().map(|c| format!("\t- {}", c.0)).intersperse(String::from("\n")).collect();

                info!("CHIP-8 Built-in ROMs:\n{}", builtin_list);
                return;
            }

            if rom.is_empty() { error!("A ROM (--rom <rom_path or builtin>) must be provided."); return; }
            if clock_speed > 1000 { error!("clock_speed cannot be greater than 1000."); return; }
            if display_scale > 32 { error!("display_scale cannot be greater than 32."); return; }

            chip8::Cpu::start(rom.as_str(), clock_speed as f64, display_scale);
        },

        _ => error!("Not yet implemented")
    }
}