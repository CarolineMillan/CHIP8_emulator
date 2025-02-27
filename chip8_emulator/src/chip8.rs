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
            // sprite pixels in memory are XORed onto the screen

            // first get x and y coordinates
            // I've hard-coded the screen size here, perhaps change this
            /*
            let xi: usize = opcode.x.into();
            let yi: usize = opcode.y.into();
            let x_coord = self.v_reg[xi] % 64;
            let y_coord = self.v_reg[yi] % 32;
            */

            let mut x_coord = self.v_reg[opcode.x as usize] % 64;
            let mut y_coord = self.v_reg[opcode.y as usize] % 32;

            //set VF to 0
            self.v_reg[16] = 0;


            let mut on = false;
            let mut curr_x = x_coord;
            let mut curr_y = y_coord;

            //for n rows (starting at memory address stored in I)
            for number in 0..(opcode.n-1) {
                // so to access the memory address, we want to use i = index + number - 1
                let i = self.index + number as u16;

                // get the nth byte of sprite data from this address
                let nth_sprite = self.memory.data[i as usize];

                curr_y = y_coord + number;

                
                //stop if you reach bottom edge of the screen
                if curr_y > 32 {break}

                // for each of the 8 pixels in the sprite row
                // xor them onto the screen
                for bit_index in 0..8 {
                    
                    curr_x = x_coord + bit_index;

                    //if you reach right edge of the screen stop drawing this row (break loop)
                    if curr_x > 64 {break}

                    //if sprite_pix is on and screen_pix is on
                    // turn off screen_pix and set VF to 1
                    let sprite_pix = (nth_sprite >> (7 - bit_index)) & 1;

                    if sprite_pix == 1 {
                        // get the x and y values right
                        on = self.display.bitwise_and(curr_x as u16, curr_y as u16);

                        if on {self.v_reg[16] = 1}

                    }

                    // if sprite_pix is on and screen_pix is off, draw pix on screen at x,y coords

                }
            }
            //remember to render!
            self.display.render().unwrap();
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
    - go throguh every time you've used unwrap and actually deal with results/errors


    for IBM logo: 

    00E0 (clear screen) TICK
    1NNN (jump)
    6XNN (set register VX)
    7XNN (add value to register VX)
    ANNN (set index register I)
    DXYN (display/draw)

    ALL OPCODES:
    
     */