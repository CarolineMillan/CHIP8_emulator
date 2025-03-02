// chip8 struct 
// implements fetch - decode - execute cycle using opcode

use crate::memory::Memory;
use crate::timer::Timer;
use crate::opcode::Opcode;
//use crate::input::Input;
//use crate::sound::Sound;

use std::fs;
use std::io;

#[derive(Debug)]
pub struct Chip8 {
        memory: Memory,         // 4KB RAM, fontset, etc.
        pub display: [[bool; 32]; 64], // 64x32 screen
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
        pub fn new() -> Self {
            println!("in chip8, new");
            Chip8 {
                memory: Memory::new(),         // 4KB RAM, fontset, etc.
                display: [[false; 32]; 64],    // 64x32 screen
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
            let program = fs::read(file_path)?;

            // load into memory
            self.memory.load_program(&program);

            // returns ok if there were no errors
            Ok(())
        }

        pub fn run_cycle_once(&mut self) {
            let current_opcode = self.fetch();
            self.decode_execute(current_opcode);
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

            let opcode = Opcode::new(first_byte, second_byte);

            opcode
        }

        fn decode_execute(&mut self, opcode: Opcode) {
            // decode the instruction and execute it
            // the instructions are simple enough that executing it will be a one line call to a separate method
            // match statement

            let current_op = opcode.opcode;

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
                _ => println!("Unimplemented opcode: {:04X}", current_op),
                }
        }

        fn clear_display(&mut self) {
            // set all pixels in the display to 0 -- this doesn't need to be a separate function
            self.display = [[false; 32]; 64];
        }

        fn jump(&mut self, opcode: Opcode) {
            // takes opcode 0x1NNN and jumps program counter to 0xNNN
            self.program_counter = opcode.nnn as u16;
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

            println!("in chip8, draw");

            // first get x and y coordinates
            // I've hard-coded the screen size here, perhaps change this
            let x_coord = self.v_reg[opcode.x as usize] % 64;
            let y_coord = self.v_reg[opcode.y as usize] % 32;

            //set VF to 0
            self.v_reg[15] = 0;

            //for n rows (starting at memory address stored in I)
            for number in 0..(opcode.n-1) {
                // so to access the memory address, we want to use i = index + number - 1
                let i = self.index + number as u16;

                // get the nth byte of sprite data from this address
                let nth_sprite = self.memory.data[i as usize];

                let curr_y = y_coord + number;

                //stop if you reach bottom edge of the screen
                if curr_y >= 32 {break}

                // for each of the 8 pixels in the sprite row
                // xor them onto the screen
                for bit_index in 0..8 {
                    
                    let curr_x = x_coord + bit_index;

                    //if you reach right edge of the screen stop drawing this row (break loop)
                    if curr_x >= 64 {break}

                    let sprite_pix = (nth_sprite >> (7 - bit_index)) & 1;

                    if sprite_pix == 1 {

                        // turn off screen_pix and set VF to 1
                        if self.display[curr_x as usize][curr_y as usize] {self.v_reg[15] = 1}
                        self.display[curr_x as usize][curr_y as usize] ^= true;
                    }
                }
            }
        }
    }



    /* 
    
    TODO:
    - add all opcodes to match statement
    - create functions for all opcodes
    - error handling
    - hard-coded screen size
    - get IBM logo working
    - add in timer to the main loop (probably in app.rs)
    - tidy up draw function (x's and y's)


    for IBM logo: 

    00E0 (clear screen) TICK
    1NNN (jump)
    6XNN (set register VX)
    7XNN (add value to register VX)
    ANNN (set index register I)
    DXYN (display/draw)

    ALL OPCODES:
    
     */