mod logger;
mod chip8;
mod utility;

fn main() {
    #[cfg(not(debug_assertions))]
    utility::hook_panic();

    chip8::Cpu::start("ibm", 500., 16);
}
