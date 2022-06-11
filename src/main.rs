mod logger;
mod chip8;
mod cli;
mod utility;

fn main() {
    #[cfg(not(debug_assertions))]
    utility::hook_panic();

    cli::run();
}
