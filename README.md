# CHIP-8 Emulator in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![rustc](https://img.shields.io/badge/rustc-1.84%2B-brightgreen.svg)](https://www.rust-lang.org/tools/install)

A CHIP-8 emulator (opcode interpreter) written in Rust. Currently, it:

- **Implements all 35 original opcodes**, so it runs all classic CHIP-8 programs and passes Timendus' test suite
- **Renders graphics** on a 64×32 monochrome display
- **Handles input** via a 16‑key hex keypad mapping
- **Handles audio output**
- **Has a built-in timer** so that ROMs run at the correct speed

![video_demo](images/demo.mov)

## Quick Start

```bash
git clone --recursive https://github.com/CarolineMillan/CHIP8_emulator.git
cd CHIP8_emulator 
cargo build 
cargo run rom/chip8Archive/roms/glitchGhost.ch8
```

awsd to move. e to haunt. Aim of the game is to chase all humans into their grave and haunt them.

## ROM Compatibility

This emulator supports **classic CHIP-8** only. It is not compatible with SUPER-CHIP, CHIP-48, or XO-CHIP ROMs.

## Motivation

I built this to deepen my understanding of computer architecture through hands‑on work rather than just lectures or books.

This particular project appealed to me because of the wealth of resources available online. I found [Tobias Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) especially clear and it gave the project a well-defined scope and clear execution plan, which is exactly what I was looking for when returning from a break.

I chose Rust for a fresh start in a modern systems language.

## Installation & Running the Emulator

### Clone the repo

``` git clone --recursive https://github.com/CarolineMillan/CHIP8_emulator.git ```

### Go to the project directory

``` cd CHIP8_emulator ```

### Build

``` cargo build ```

### Run with a ROM file

``` cargo run [ROM_FILEPATH] ```

This emulator currently passes all of the tests in Timendus' test suite for CHIP-8, so these should work without modification.

I am in the process of trying out the CHIP-8 games in John Earnest's CHIP-8 Archive. Make sure it's a CHIP-8 ROM and not a SUPER-CHIP or XO-CHIP ROM before running it.

I have added both of these repositories as Git submodules in this project.

You can run any compatible CHIP-8 ROM. Just replace [ROM_FILEPATH] with the path to your ROM file.

## Controls

Press `Esc` to quit the emulator at any time.

Adjust CYCLES_PER_FRAME in main.rs to speed up or slow down emulation. This is set to 10 by default and works well for most ROMs. You can try values between 8–16 if needed.

### Keypad layout

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

I am currently finished with this project, but may come back one day to:

- Extend to a CHIP-48, SUPER-CHIP or XO-CHIP
- Try making a CHIP-8 ROM using [Octo](https://johnearnest.github.io/Octo/index.html?key=VSNszvkc)

## Acknowledgements and Resources Used

- [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) was my main resource for the project.
- [Cowgod's reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) was my main resource for the opcodes.
- [Timendus' test suite](https://github.com/Timendus/chip8-test-suite) was my main resource for testing.
- [Chip8Archive](https://github.com/JohnEarnest/chip8Archive) for further testing.
- [ChatGPT](https://chatgpt.com/) helped with debugging and improving README structure and clarity. However, it was often unreliable, especailly for broader advice/design decisions: frequently suggesting incompatible libraries and giving problematic advice around lifetimes, self-referential structs, and architectural design that complicated development rather than simplifying it. This was my first project since generative AI became widely available, and I learnt a lot about it's strengths and weaknesses. I may write a blog post on this in future.

## Licence

This software is available as open source under the terms of [the MIT License](https://opensource.org/license/MIT).


[def]: images/demo.mov