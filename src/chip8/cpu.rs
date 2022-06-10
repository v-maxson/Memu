//! This module contains the CPU for the CHIP-8.

use fixedstep::FixedStep;
use pixels::{SurfaceTexture, Pixels};
use smallvec::{SmallVec, smallvec};
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, dpi::LogicalSize, event::{Event, WindowEvent}};
use crate::{error, utility::get_rgba, warn};
use super::{BUILTINS, Instruction, INSTRUCTION_TABLE};

pub const MEMORY_SIZE: usize = 0x1000;
pub const V_REG_COUNT: usize = 0x10;
pub const STACK_SIZE: usize = 0x10;
pub const PC_START: usize = 0x200;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_DIMENSIONS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;
pub const PIXEL_ON: u32 = 0xFFFFFFFF;
pub const PIXEL_OFF: u32 = 0x00000000;

pub const FONTSET: &[u8] = include_bytes!("../../resources/chip8/fontset");
pub const FONTSET_START: usize = 0x80;

pub struct Cpu {
    /// The CHIP-8 had 4KB of memory available to programs.
    /// 
    /// The original CHIP-8 interpreter was stored (in its entirety) 
    /// in the first 512 bytes of this memory space. In this implemenation,
    /// only the base fontset will be stored in this memory, and the rest
    /// will simply be empty. 
    pub memory: [u8; MEMORY_SIZE],

    /// The display memory for the CHIP-8.
    pub display_memory: [u32; DISPLAY_DIMENSIONS],

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

impl Default for Cpu {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            display_memory: [PIXEL_OFF; DISPLAY_DIMENSIONS],
            v: [0; V_REG_COUNT],
            st: 0,
            dt: 0,
            pc: PC_START as u16,
            i: 0,
            stack: smallvec![0; STACK_SIZE]
        }
    }
}

impl Cpu {
    pub fn start(rom_path: &str, clock_speed_hz: f64, display_scale: u8) {
        let rom: Vec<u8>;
        // Search for the rom in the set of builtins.
        if let Some(r) = BUILTINS.get(rom_path.to_lowercase().as_str()) {
            rom = r.to_vec();
        } else {
            // Search for ROM as a path.
            let path = std::path::Path::new(rom_path);
            if !path.exists() {
                error!("Provided ROM path does not exist. If this is incorrect, this could be due to missing permissions.");
                panic!();
            } else {
                match std::fs::read(rom_path) {
                    Ok(vec) => {
                        if vec.len() > MEMORY_SIZE - PC_START {
                            error!("Selected ROM is too large to fit into CHIP-8 Memory Space.");
                            panic!();
                        } else { rom = vec }
                    },
                    Err(err) => {
                        error!("Could not read provided ROM: {}", err);
                        panic!();
                    }
                }
            }
        }

        let mut cpu = Cpu::default();

        // Load selected ROM into memory.
        for (i, byte) in rom.iter().enumerate() {
            cpu.memory[PC_START + i] = *byte;    
        }
        drop(rom);

        // Load fontset into memory.
        for (i, byte) in FONTSET.iter().enumerate() {
            cpu.memory[FONTSET_START + i] = *byte;
        }

        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new((DISPLAY_WIDTH) as f64, (DISPLAY_HEIGHT) as f64);
            WindowBuilder::new()
                .with_title("Memu: CHIP-8")
                .with_inner_size(size)
                .with_resizable(false)
                .build(&event_loop)
                .unwrap()
        };
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, &window);
            Pixels::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, surface_texture).unwrap()
        };

        // Scale window
        window.set_inner_size(LogicalSize::new((DISPLAY_WIDTH * display_scale as usize) as f64, (DISPLAY_HEIGHT * display_scale as usize) as f64));
        
        let mut cpu_step = FixedStep::start(clock_speed_hz);
        let mut timer_step = FixedStep::start(60.);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    // Update the current frame with data from the buffer.
                    let frame = pixels.get_frame();
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        // Get the pixel from the buffer.
                        let buffer_pixel = get_rgba(cpu.display_memory[i]);
                        pixel[0] = buffer_pixel.0;
                        pixel[1] = buffer_pixel.1;
                        pixel[2] = buffer_pixel.2;
                        pixel[3] = buffer_pixel.3;
                    }
                
                    let render_resulted_in_err = {
                        pixels
                            .render_with(|encoder, target, context| {
                                context.scaling_renderer.render(encoder, target);
                                Ok(())
                            })
                            .map_err(|e| error!("Render Failed with: {e}"))
                            .is_err()
                    };
                
                    if render_resulted_in_err {
                        *control_flow = ControlFlow::Exit;
                        panic!();
                    }
                }
                
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                            panic!();
                        },

                        WindowEvent::Resized(size) => {
                            pixels.resize_surface(size.width, size.height);
                        }

                        _ => ()
                    }
                }

                _ => ()
            }

            while cpu_step.update() {
                // Decode the current instruction in memory.
                let ins = Instruction::from_u8_pair(cpu.memory[cpu.pc as usize], cpu.memory[(cpu.pc + 1) as usize]);

                // Increment the PC
                cpu.pc += 2;

                // Execute the instruction.
                if let Some(func) = INSTRUCTION_TABLE.get(&ins.op) {
                    func(&mut cpu, &ins);
                } else {
                    warn!("Unrecognized Instruction: {}", ins);
                }

                if ins.op == 0xD { window.request_redraw() }
            }

            while timer_step.update() {
                if cpu.dt > 0 { cpu.dt -= 1; }
                if cpu.st > 0 {
                    // TODO: Implement Beep Sound.
                    cpu.st -= 1;
                }
            }

            *control_flow = ControlFlow::Poll;
        });

    }
}
