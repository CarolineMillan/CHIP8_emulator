# CHIP8_emulator
This is an interpreter for CHIP-8 programs.

## Installation

### Clone the repo:
``` git clone https://github.com/CarolineMillan/CHIP8_emulator.git ```

### Go to the project directory:
``` cd CHIP8_emulator ``` 

``` cd chip8_emulator ```

### Build:
``` cargo build ```

### Run with a ROM file:
``` cargo run [ROM_FILEPATH] ```

## Config
### Keypad layout:

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

Escape to quit the emulator.

Adjust CYCLES_PER_FRAME in main.rs to speed up or slow down emulation. It should be set between 8-16 for most ROMS.

-------------------------------------

### Roadmap/ideas for future improvements or extensions:
- separate host-specific timing fields from the chip8 struct
- extend to a CHIP-48 or SUPER-CHIP, and allow the user to toggle between the three versions

-------------------------------------

### Acknowledgements and Resources Used:
- [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) was my main resource for the project.
- [Cowgod's reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) was my main resource for the opcodes.
- [Timendus' test suite](https://github.com/Timendus/chip8-test-suite) was my main resource for testing.

--------------------------------------

### Licence

This software is available as open source under the terms of [the MIT License](https://opensource.org/license/MIT).
