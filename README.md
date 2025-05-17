# CHIP-8_emulator
My CHIP-8 Emulator in Rust.

This is an interpreter for CHIP-8 programs.

[why did you build this -- to learn about computer architecture, learning low level emulation, learning rust, first project back after a break. a well-defined project to start with -- it was basically just following the instructions]

-------------------------------------

How to run the CHIP-8 interpreter:

-------------------------------------

Features:

-------------------------------------

Design Decisions:
- explain module choices
- I could have planned this project more. I did jump straight in without too much planning and this meant refactoring the code and changing the original design a few times throughout. However, I think this was the best decision for this project, as it lowered the barrier to getting started again after being off sick.
- I probably made it more object oriented than it needed to be, but I found having an opcode object was useful for my learning to help me visualise/understand how opcodes are broken down and bit manipulation. This could probably be tidied up.
- actually the whole thing could probably be tidied up now that I'm familiar with the concepts
- challenges 
    - graphics (pixels and winit didn't work but I spent ages trying to get it to work!! I'm still not convinced it's possible with the lifetime issue, without restructuring my code in a way that is unfaithful to chip-8...) (using minifb worked a charm, so much simpler)
- why rust? something new, wanted something completely different/a fresh start. I've stuck with it because... [I do enjoy it, I like cargo and the compiler errors, and also I've heard that some c++ users are moving to Rust, so it might come in useful]

-------------------------------------

I tested my interpreter using [_Timendus' test suite_](https://github.com/Timendus/chip8-test-suite).

-------------------------------------

Roadmap/ideas for future improvements or extensions:
- add a timer
- add sound
- write a script to generate pixel art roms 
- get it running on an ESP32 (might be a good first embedded project...)
- try a more advanced emulation (Z80, gameboy advanced), an app that lets you play gameboy games could be cool

-------------------------------------

Acknowledgements and Resources Used:
- [_Tobias V. Langhoff's guide_](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) was my main resource for the project 
- [_Cowgod's reference_](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) was my main resource for the opcodes 

--------------------------------------

pick a licence