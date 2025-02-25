// chip8 struct 
// implements fetch - decode - execute cycle using opcode
// opcode functions are in here, not in opcode.rs

// got rid of opcode.rs

//use crate::display;
use crate::display::Display;
use crate::memory::Memory;
use crate::timer::Timer;
//use crate::input::Input;
//use crate::sound::Sound;

use std::fs;
use std::io;

pub struct Chip8<'a> {
        memory: Memory,         // 4KB RAM, fontset, etc.
        display: Display<'a>,   // 64x32 screen
        program_counter: u16,   // Program Counter (0x200-0xFFF)
        index: u16,             // Index Register
        stack: [u16; 16],       // 16-level call stack
        stack_pointer: u8,      // Stack Pointer
        v_reg: [u8; 16],        // Variable registers V0-VF
        delay_timer: Timer,     // Countdown timer
        sound_timer: Timer,     // Beep timer
        keypad: [bool; 16],     // State of the 16 CHIP-8 keys
    }

    impl<'a> Chip8<'a> {
        pub fn new(display: Display<'a>) -> Self {
            Chip8 {
                memory: Memory::new(),         // 4KB RAM, fontset, etc.
                display: display, //Display::new(),       // 64x32 screen
                program_counter: 0x200,        // Program Counter (0x200-0xFFF)
                index: 0,                      // Index Register
                stack: [0; 16],                // 16-level call stack
                stack_pointer: 0,              // Stack Pointer
                v_reg: [0; 16],                // Variable registers V0-VF
                delay_timer: Timer::new(0),     // Countdown timer
                sound_timer: Timer::new(0),     // Beep timer
                keypad: [false; 16],           // State of the 16 CHIP-8 keys
            }
        }

        pub fn load_program(&mut self, file_path: &str) -> io::Result<()>{
            //loads a program into the memory

            //read the file in filepath (it will be a Vec<u8>)
            // I need to double check the type and use of ? here 
            let prog = fs::read(file_path)?;

            // load into memory
            self.memory.load_program(&prog);

            // returns ok if there were no errors
            Ok(())
        }

        pub fn run_cycle(&mut self) {
            // 

        }

        fn update_timers(&mut self) {
            self.delay_timer.tick();
            self.sound_timer.tick();
        }

        fn fetch(&mut self) -> u16 {
            // fetch the instruction from memory at the current PC

            // an opcode is 2 bytes, so need to read 2 bytes
            let first_byte = self.memory.read_byte(self.program_counter.into());
            let second_byte = self.memory.read_byte((self.program_counter+1).into());

            //increment the program_counter -- maybe do this in the main func
            self.program_counter += 2;

            //combine the bytes into one output
            //chip8 opcodes are stored in Big-Endian
            // Zilog Z80 uses Little-Endian, so just change to from_le_bytes for this 
            u16::from_be_bytes([first_byte, second_byte])
        }

        fn decode_execute() {
            // decode the instruction and execute it
            // the instructions are simple enough that executing it will be a one line call to a separate method

        }
    }