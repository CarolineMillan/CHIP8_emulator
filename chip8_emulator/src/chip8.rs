// chip8 struct 
// implements fetch - decode - execute cycle using opcode

//use crate::display;
use crate::display::Display;
use crate::memory::Memory;
use crate::timer::Timer;
use crate::opcode::Opcode;
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
                delay_timer: Timer::new(60),     // Countdown timer
                sound_timer: Timer::new(60),     // Beep timer
                keypad: [false; 16],           // State of the 16 CHIP-8 keys
            }
        }

        pub fn load_program(&mut self, file_path: &str) -> io::Result<()>{
            //loads a program into the memory

            //read the file in filepath (it will be a Vec<u8>)
            // I need to double check the type and use of ? here 
            let program = fs::read(file_path)?;

            // load into memory
            self.memory.load_program(&program);

            // returns ok if there were no errors
            Ok(())
        }

        pub fn run_cycle(&mut self) {
            // runs the program

            // while program_counter != 0xFFF (what is 0xFFF +1?)

            //fetch the opcode
            let current_opcode = self.fetch();

            // split opcode into nibbles before decoding
            //let nibbles = self.split(current_opcode);

            //decode_execute the opcode
            self.decode_execute(current_opcode);

            //increment program_counter -- no this is done in fetch to avoid errors
            //self.program_counter += 2;

            //repeat

        }

        fn update_timers(&mut self) {
            self.delay_timer.tick();
            self.sound_timer.tick();
        }

        fn fetch(&mut self) -> Opcode {
            // fetch the instruction from memory at the current PC

            // an opcode is 2 bytes, so need to read 2 bytes
            let first_byte = self.memory.read_byte(self.program_counter.into());
            let second_byte = self.memory.read_byte((self.program_counter+1).into());

            //increment the program_counter -- do this here to avoid errors
            self.program_counter += 2;

            //combine the bytes into one output
            //chip8 opcodes are stored in Big-Endian
            // Zilog Z80 uses Little-Endian, so just change to from_le_bytes for this 
            //u16::from_be_bytes([first_byte, second_byte]);

            //UNLESS
            // we don't want a u16. we want a new Opcode.
            let opcode = Opcode::new(first_byte, second_byte);

            opcode
        }

        fn decode_execute(&mut self, opcode: Opcode) {
            // decode the instruction and execute it
            // the instructions are simple enough that executing it will be a one line call to a separate method
            // match statement

            // decide whether to store the current opcode in the struct rather than pass it as input to most functions

            let current_op = opcode.opcode;

            match current_op {
                0x00E0 => self.clear_display(),
                0x1___ => self.jump(opcode),
                0x6___ => self.set_register(opcode),
                0x7___ => self.add_to_register(opcode),
                0xA___ => self.set_index_register(opcode),
                0xD___ => self.draw(opcode),
                _ => todo!(),
            }

        }

        fn clear_display(&mut self) {
            // set all pixels in the display to 0 -- this doesn't need to be a separate function

            // fix this output 
            let _res = self.display.clear_display();
        }

        fn jump(&mut self, opcode: Opcode) {
            // takes opcode 0x1NNN and jumps program counter to 0xNNN
            self.program_counter = opcode.nnn;
        }

        fn set_register(&mut self, opcode: Opcode) {
            //set register VX

            let i: usize = opcode.x.into();

            self.v_reg[i] = opcode.nn;
        }

        fn add_to_register(&mut self, opcode: Opcode) {
            //add value to register VX

            let i: usize = opcode.x.into();

            self.v_reg[i] += opcode.nn;
        }

        fn set_index_register(&mut self, opcode: Opcode) {
            //sets index register 

            self.index = opcode.nnn;
        }

        fn draw(&mut self, opcode: Opcode) {

        }
    }



    /* 
    
    TODO:
    - run_cycle logic
    - write decode_execute match statement
    - make an opcode class that splits an opcode up into easily accessible nibbles
    - get opcodes and their actions
    - what to start the timer from? 60 for now
    - which functions are methods on the struct and which are just functions that can go outside of impl


    for IBM logo: 

    00E0 (clear screen) TICK
    1NNN (jump)
    6XNN (set register VX)
    7XNN (add value to register VX)
    ANNN (set index register I)
    DXYN (display/draw)

    ALL OPCODES:
    
     */