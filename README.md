# Memu
A collection of emulators made with Rust.

> Author's Note: This is mainly an exploration into the world of emulation tech. 
> While collaboration is accepted, it will be extremely limited. This project isn't 
> meant to create the best emulators out there, it's just to create them.

---

#### Emulation Targets
- [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)*
- [NES](https://en.wikipedia.org/wiki/Nintendo_Entertainment_System)**
- [Gameboy](https://en.wikipedia.org/wiki/Game_Boy)**
- [Gameboy Color](https://en.wikipedia.org/wiki/Game_Boy_Color)**

&nbsp;

> \* This target has incomplete support, check its section for missing features/issues.
> 
> \** This target is not yet implemented.

---

#### Usage
For now, this application is run from the command line.* Each emulator has settings that are specific to it, for example, the CHIP-8 interpreter has a unique `speed` argument that controls the speed at which the CPU clock runs. An example of running the CHIP-8 interpreter with the built in `IBM` program:

```sh
memu chip --rom ibm --speed 200 --scale 32
```

For more help running this program, use the built in `memu help` command.


> \* There are plans to change this in the future.

---

#### CHIP-8
This section contains information specific to the CHIP-8 portion of this project.

##### Notes:
- Includes a `speed` CLI Option as the clock speed was never standardized and could vary
  between implementations.
- Minimal built-in ROMs provided as they were only used for testing purposes. 
  More ROMs can be downloaded [here](https://github.com/kripod/chip8-roms).

&nbsp;

##### CLI Usage:
```sh
memu chip [OPTIONS]
```
- **Options:**
  - `-h, --help` -> Print help information
  - `-l` -> Print a list of built-in ROMs and exits
  - `-r, --rom <ROM>` -> The ROM to execute
  - `-s, --scale <DISPLAY_SCALE>` -> The scale of the display [default: 16]
  - `--speed <CLOCK_SPEED>` -> The CPU speed (in hz) [default: 500]

&nbsp;

##### Known Issues:
- None.

&nbsp;

##### Missing Features:
- Input
  - Instructions `0xE` and `0xF` are not yet implemented.

---