//! Contains the logic for the CLI.
#![allow(unused, unreachable_patterns)]

use clap::{Parser, Subcommand};
use crate::{error, chip8};

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
        #[clap(short, long)]
        rom: String,

        /// The CPU speed (in hz) 
        #[clap(long = "speed", default_value_t = 500)]
        clock_speed: u16,

        /// The scale of the display.
        #[clap(short = 's', long = "scale", default_value_t = 16)]
        display_scale: u8
    }
}

/// Parses CLI arguments for this program.
pub fn run() {
    let args = CliArguments::parse();

    match args.command {
        Command::Chip { rom, clock_speed, display_scale } => {
            if clock_speed > 1000 { error!("clock_speed cannot be greater than 1000"); return; }
            if display_scale > 32 { error!("display_scale cannot be greater than 32"); return; }

            chip8::Cpu::start(rom.as_str(), clock_speed as f64, display_scale);
        },

        _ => error!("Not yet implemented")
    }
}