# CHIP-8 Emulator in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![rustc](https://img.shields.io/badge/rustc-1.84%2B-brightgreen.svg)](https://www.rust-lang.org/tools/install)

A CHIP-8 emulator (opcode interpreter) written in Rust. Currently, it: 
- **Implements all 35 original opcodes**, so it runs all classic CHIP-8 programs and passes Timendus' test suite
- **Renders graphics** on a 64×32 monochrome display
- **Handles input** via a 16‑key hex keypad mapping

## Quick Start

```bash
git clone --recursive https://github.com/CarolineMillan/CHIP8_emulator.git
cd CHIP8_emulator 
cargo build 
cargo run [ROM_FILEPATH]
```

## ROM Compatibility

This emulator supports **classic CHIP-8** only. It is not compatible with SUPER-CHIP, CHIP-48, or XO-CHIP ROMs.


## Motivation

I built this to deepen my understanding of computer architecture through hands‑on work rather than just lectures or books. 

This particular project appealed to me because of the wealth of resources available online. I found [Tobias Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) especially clear and it gave the project a well-defined scope and clear execution plan, which is exactly what I was looking for when returning from a break. 

I chose Rust both to get a fresh start in a modern systems language and to push myself with a new toolchain.

## Installation & Running the Emulator

### Clone the repo:
``` git clone --recursive https://github.com/CarolineMillan/CHIP8_emulator.git ```

### Go to the project directory:
``` cd CHIP8_emulator ```

### Build:
``` cargo build ```

### Run with a ROM file:
``` cargo run [ROM_FILEPATH] ```

This emulator currently passes all of the tests in Timendus' test suite for CHIP-8, so these should work without modification. 

I am in the process of trying out the CHIP-8 games in John Earnest's CHIP-8 Archive. Make sure it's a CHIP-8 ROM and not a SUPER-CHIP or XO-CHIP ROM before running it.

I have added both of these repositories as Git submodules in this project.

You can run any compatible CHIP-8 ROM. Just replace [ROM_FILEPATH] with the path to your ROM file.

## Controls 

Press `Esc` to quit the emulator at any time.

Adjust CYCLES_PER_FRAME in main.rs to speed up or slow down emulation. This is set to 10 by default and works well for most ROMs. You can try values between 8–16 if needed.

### Keypad layout:

This is the key mapping that the emulator uses. It is non-configurable for now.

|Key | CHIP-8|
|----|-------|
|1|1|
|2|2|
|3|3|
|4|C|
|Q|4|
|W|5|
|E|6|
|R|D|
|A|7|
|S|8|
|D|9|
|F|E|
|Z|A|
|X|0|
|C|B|
|V|F|

## Future Plans

### Short-term

- Finalise error handling
- Decouple host-specific timing from the chip8 struct (would be nice to completely separate emulator logic from frontend).

### Long-term

- Extend to a CHIP-48, SUPER-CHIP or XO-CHIP
- Try making a CHIP-8 ROM using [Octo](https://johnearnest.github.io/Octo/index.html?key=VSNszvkc)

## Acknowledgements and Resources Used:

- [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) was my main resource for the project.
- [Cowgod's reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) was my main resource for the opcodes.
- [Timendus' test suite](https://github.com/Timendus/chip8-test-suite) was my main resource for testing.
- [Chip8Archive](https://github.com/JohnEarnest/chip8Archive) for further testing.
- [ChatGPT](https://chatgpt.com/) helped with debugging opcode implementations and improving README structure and clarity. However, its broader project guidance was often unreliable: frequently suggesting incompatible Rust libraries and introducing problematic advice around lifetimes, self-referential structs, and architectural design that complicated development rather than simplifying it.

## Licence

This software is available as open source under the terms of [the MIT License](https://opensource.org/license/MIT).
