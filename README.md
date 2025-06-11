# CHIP8_emulator
My CHIP-8 Emulator in Rust.

This is an interpreter for CHIP-8 programs.

------------------------------------

Installation

Clone the repo:
git clone https://github.com/CarolineMillan/CHIP8_emulator.git

Go to the project directory:
cd CHIP8_emulator
cd chip8_emulator

Build:
cargo build 

Run with a ROM file:
cargo run [ROM_FILEPATH]

Keypad layout:

Key || CHIP-8
1 || 0x1
2 || 0x2
3 || 0x3
4 || 0xC
Q || 0x4
W || 0x5
E || 0x6
R || 0xD
A || 0x7
S || 0x8
D || 0x9
F || 0xE
Z || 0xA
X || 0x0
C || 0xB
V || 0xF

Escape to quit the emulator.

Adjust CYCLES_PER_FRAME in main.rs to speed up or slow down emulation. It should be set between 8-16 for most ROMS.

-------------------------------------

Roadmap/ideas for future improvements or extensions:
- separate host-specific timing fields from the chip8 struct
- extend to a CHIP-48 or SUPER-CHIP, adn allow the user to toggle between the three versions

-------------------------------------

Acknowledgements and Resources Used:
- [_Tobias V. Langhoff's guide_](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) was my main resource for the project.
- [_Cowgod's reference_](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) was my main resource for the opcodes.
- [_Timendus' test suite_](https://github.com/Timendus/chip8-test-suite) was my main resource for testing.

--------------------------------------

pick a licence