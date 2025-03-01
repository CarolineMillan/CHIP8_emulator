// chip8 struct 
// implements fetch - decode - execute cycle using opcode

//use crate::display;
use crate::display::Display;
use crate::memory::Memory;
use crate::timer::Timer;
use crate::opcode::Opcode;
//use crate::input::Input;
//use crate::sound::Sound;

use winit::window::Window;

use std::fmt::LowerHex;
use std::fs;
use std::io;

//[derive #debug]
#[derive(Debug)]
pub struct Chip8 {
        memory: Memory,         // 4KB RAM, fontset, etc.
        pub display: [[bool; 32]; 64], //Display<'a>,   // 64x32 screen
        program_counter: u16,   // Program Counter (0x200-0xFFF)
        index: u16,             // Index Register
        stack: [u16; 16],       // 16-level call stack
        stack_pointer: u8,      // Stack Pointer
        v_reg: [u8; 16],        // Variable registers V0-VF
        delay_timer: Timer,     // Countdown timer
        sound_timer: Timer,     // Beep timer
        keypad: [bool; 16],     // State of the 16 CHIP-8 keys
    }

    impl Chip8 {
        //pub fn new(display: Display<'a>) -> Self {
        pub fn new() -> Self {
            println!("in chip8, new");
            Chip8 {
                memory: Memory::new(),         // 4KB RAM, fontset, etc.
                display: [[false; 32]; 64],//Display::new(window), //display, //Display::new(),       // 64x32 screen
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

            println!("in chip8, load program");

            //read the file in filepath (it will be a Vec<u8>)
            // I need to double check the type and use of ? here 
            let program = fs::read(file_path)?;

            // load into memory
            self.memory.load_program(&program);

            //println!("in chip8, load program 2");

            // returns ok if there were no errors
            Ok(())
        }

        pub fn run_cycle(&mut self) {
            // runs the program

            //println!("in chip8, run cycle");

            //while self.program_counter <= 0xFFF {

                //println!("in chip8, run cycle 2");

                //fetch the opcode
                let current_opcode = self.fetch();

                //println!("in chip8, run cycle 3");

                //decode_execute the opcode
                self.decode_execute(current_opcode);
            //}

            //println!("in chip8, run cycle 4");

        }

        fn update_timers(&mut self) {
            self.delay_timer.tick();
            self.sound_timer.tick();
        }

        fn fetch(&mut self) -> Opcode {
            // fetch the instruction from memory at the current PC

            //println!("in chip8, fetch");

            // an opcode is 2 bytes, so need to read 2 bytes
            let first_byte = self.memory.read_byte(self.program_counter.into());
            let second_byte = self.memory.read_byte((self.program_counter+1).into());

            //println!("in chip8, fetch 2");

            //println!("1b: {}, 2b: {}", first_byte, second_byte);

            //increment the program_counter -- do this here to avoid errors
            self.program_counter += 2;


            //UNLESS
            // we don't want a u16. we want a new Opcode.
            let opcode = Opcode::new(first_byte, second_byte);

            //println!("in chip8, fetch 3");

            opcode
        }

        fn decode_execute(&mut self, opcode: Opcode) {
            // decode the instruction and execute it
            // the instructions are simple enough that executing it will be a one line call to a separate method
            // match statement

            println!("in chip8, decode execute");

            // decide whether to store the current opcode in the struct rather than pass it as input to most functions

            let current_op = opcode.opcode;

            println!("opcode: {}", opcode.opcode);

            //bitmask!
            match opcode.opcode & 0xF000 {
                0x0000 => {
                    if opcode.opcode == 0x00E0 {
                        self.clear_display()
                    } else {
                        // Handle other 0x0 opcodes if needed
                    }
                },
                0x1000 => self.jump(opcode),
                0x6000 => self.set_register(opcode),
                0x7000 => self.add_to_register(opcode),
                0xA000 => self.set_index_register(opcode),
                0xD000 => self.draw(opcode),
                _ => todo!(),
            }
/* 
            match current_op {
                0x00E0 => self.clear_display(),
                0x1___ => self.jump(opcode),
                0x6___ => self.set_register(opcode),
                0x7___ => self.add_to_register(opcode),
                0xA___ => self.set_index_register(opcode),
                0xD___ => self.draw(opcode),
                _ => todo!(),
            }
*/
            //println!("in chip8, decode execute 2");

        }

        fn clear_display(&mut self) {
            // set all pixels in the display to 0 -- this doesn't need to be a separate function
            //println!("in chip8, clear_display");
            //let _res = self.display.clear_display();
            self.display = [[false; 32]; 64];
        }

        fn jump(&mut self, opcode: Opcode) {
            // takes opcode 0x1NNN and jumps program counter to 0xNNN
            //println!("in chip8, jump");
            self.program_counter = opcode.nnn;
        }

        fn set_register(&mut self, opcode: Opcode) {
            //set register VX

            //println!("in chip8, set register");

            let i: usize = opcode.x.into();

            self.v_reg[i] = opcode.nn;
        }

        fn add_to_register(&mut self, opcode: Opcode) {
            //add value to register VX

            //println!("in chip8, add to register");

            let i: usize = opcode.x.into();

            self.v_reg[i] += opcode.nn;
        }

        fn set_index_register(&mut self, opcode: Opcode) {
            //sets index register 

            //println!("in chip8, set index register");

            self.index = opcode.nnn;
        }

        fn draw(&mut self, opcode: Opcode) {
            // sprite pixels in memory are XORed onto the screen

            //println!("in chip8, draw");

            // first get x and y coordinates
            // I've hard-coded the screen size here, perhaps change this
            let x_coord = self.v_reg[opcode.x as usize] % 64;
            let y_coord = self.v_reg[opcode.y as usize] % 32;

            //set VF to 0
            self.v_reg[15] = 0;

//            println!("in chip8, draw 2");

            //for n rows (starting at memory address stored in I)
            for number in 0..(opcode.n-1) {

                //println!("in chip8, draw 3");
                // so to access the memory address, we want to use i = index + number - 1
                let i = self.index + number as u16;

                // get the nth byte of sprite data from this address
                let nth_sprite = self.memory.data[i as usize];

                let curr_y = y_coord + number;

                
                //stop if you reach bottom edge of the screen
                if curr_y > 32 {break}

                // for each of the 8 pixels in the sprite row
                // xor them onto the screen
                for bit_index in 0..8 {

                    //println!("in chip8, draw 4");
                    
                    let curr_x = x_coord + bit_index;

                    //if you reach right edge of the screen stop drawing this row (break loop)
                    if curr_x > 64 {break}

                    let sprite_pix = (nth_sprite >> (7 - bit_index)) & 1;

                    if sprite_pix == 1 {
                        //println!("in chip8, draw 5");
                        // get the x and y values right
                        let on = self.display[curr_x as usize][curr_y as usize] & true;
                        //let on = self.display.bitwise_and(curr_x as u16, curr_y as u16);

                        // turn off screen_pix and set VF to 1
                        if on {self.v_reg[15] = 1}

                    }
                }
            }

            //println!("in chip8, draw 6");
            //remember to render!
            //let _ = self.display.render();
            //println!("in chip8, draw 7");
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