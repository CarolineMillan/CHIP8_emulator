// chip8 struct 
// implements fetch - decode - execute cycle using opcode

use crate::memory::Memory;
use crate::opcode::Opcode;


use std::fs;
use std::io;
use std::u8;
use rand::Rng; //random number generator
use rand::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
//const SCALE: usize = 10;



#[derive(Debug)]
pub struct Chip8 {
        memory: Memory,         // 4KB RAM, fontset, etc.
        pub display: [bool; WIDTH*HEIGHT], // 64x32 screen
        program_counter: u16,   // Program Counter (0x200-0xFFF)
        index: u16,             // Index Register
        stack: [u16; 16],       // 16-level call stack
        stack_pointer: u8,      // Stack Pointer
        v_reg: [u8; 16],        // Variable registers V0-VF
        delay_timer: u8,//Timer,     // Countdown timer
        sound_timer: u8,//Timer,     // Beep timer
        keypad: [bool; 16],     // State of the 16 CHIP-8 keys
    }

    impl Chip8 {
        pub fn new() -> Self {
            println!("in chip8, new");
            Chip8 {
                memory: Memory::new(),         // 4KB RAM, fontset, etc.
                display: [false; WIDTH*HEIGHT],    // 64x32 screen
                program_counter: 0x200,        // Program Counter (0x200-0xFFF)
                index: 0,                      // Index Register
                stack: [0; 16],                // 16-level call stack
                stack_pointer: 0,              // Stack Pointer
                v_reg: [0; 16],                // Variable registers V0-VF
                delay_timer: 0,//Timer::new(60),     // Countdown timer
                sound_timer: 0,//Timer::new(60),     // Beep timer
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

        pub fn update_keypad(&mut self, key: usize, value: bool) {
            self.keypad[key] = value;
            println!("UPDATING KEYPAD!!!");
        }

        pub fn run_cycle_once(&mut self) {
            let current_opcode = self.fetch();
            self.decode_execute(current_opcode);
        }
/*
        fn update_timers(&mut self) {
            self.delay_timer.tick();
            self.sound_timer.tick();
        }
*/
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

            match opcode.a {
                0x0 => {
                    match opcode.nn {
                        0xE0 => self.clear_display(),
                        0xEE => self.return_from_subroutine(),
                        _ => println!("Unimplemented opcode: {:04X}", current_op),
                    }
                },
                0x1 | 0xB => self.jump(opcode),
                0x2 => self.call(opcode),
                0x3 | 0x4 | 0x5 | 0x9 | 0xE => self.skip(opcode),
                0x6 | 0x8 | 0xC => self.set_register(opcode),
                0x7 => self.add_to_register(opcode),
                0xA => self.set_index_register(opcode),
                0xD => self.draw(opcode),
                0xF => match opcode.nn {
                    0x07 => self.set_register(opcode),
                    0x0A => self.wait(opcode),
                    0x15 => self.set_delay_timer(opcode),
                    0x18 => self.set_sound_timer(opcode),
                    0x1E | 0x29 => self.set_index_register(opcode),
                    0x33 => self.store_bcd_mem(opcode),
                    0x55 => self.store_mem(opcode),
                    0x65 => self.read_mem(opcode),
                    _ => println!("Unimplemented opcode: {:04X}", current_op),
                }
                _ => println!("Unimplemented opcode: {:04X}", current_op),
            }
        }

        fn clear_display(&mut self) {
            // set all pixels in the display to 0
            self.display = [false; WIDTH*HEIGHT];
        }

        fn return_from_subroutine(&mut self) {
            //sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            self.program_counter = self.stack[self.stack_pointer as usize];
            self.stack_pointer -= 1;
        }

        fn jump(&mut self, opcode: Opcode) {
            // takes opcode 0x1NNN and jumps program counter to 0xNNN
            if opcode.a == 1 {
                self.program_counter = opcode.nnn as u16;
            }
            else if opcode.a == 5 {
                self.program_counter = (opcode.nnn as u16) + self.v_reg[0] as u16;
            }
            else {println!("Unimplemented opcode: {:04X}", opcode.opcode)}
        }

        fn call(&mut self, opcode: Opcode) {
            // call subroutine at nnn
            self.stack_pointer += 1;
            self.stack[self.stack_pointer as usize] = self.program_counter;
            self.program_counter = opcode.nnn;
        }

        fn skip(&mut self, opcode: Opcode) {

            match opcode.a {
                0x3 => {
                    // skip next instruction if Vx == nn
                    if self.v_reg[opcode.x as usize] == opcode.nn {self.program_counter += 2;}
                }
                0x4 => {
                    // skip next instruction if Vx != nn
                    if self.v_reg[opcode.x as usize] != opcode.nn {self.program_counter += 2;}
                }
                0x5 => {
                    // skip next instruction if Vx == Vy
                    if self.v_reg[opcode.x as usize] == self.v_reg[opcode.y as usize] {self.program_counter += 2;}
                }
                0x9 => {
                    //Skip next instruction if Vx != Vy.
                    if self.v_reg[opcode.x as usize] != self.v_reg[opcode.y as usize] {self.program_counter += 2;};
                }
                0xE => {
                    if opcode.nn == 0x9E {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                        if self.keypad[self.v_reg[opcode.x as usize] as usize] {self.program_counter +=2;}
                    } 
                    else if opcode.nn == 0xA1 {
                    //ExA1 - SKNP Vx
                    //Skip next instruction if key with the value of Vx is not pressed.
                        if !self.keypad[self.v_reg[opcode.x as usize] as usize] {self.program_counter +=2;}
                    }
                    else {
                        println!("This isn't a skip opcode!");
                    }
                    
                }
                _ => {
                    // else what?
                    println!("This isn't a skip opcode!");
                }
            }
        }

        fn set_register(&mut self, opcode: Opcode) {

            let x = opcode.x as usize;
            let y = opcode.y as usize;
            match opcode.a {
                0x6 => {
                    // set register VX
                    self.v_reg[x] = opcode.nn;
                },
                0x7 => {
                    // add value to register VX
                    self.v_reg[x] = self.v_reg[x].wrapping_add(opcode.nn);
                },
                0x8 => {
                    match opcode.n {
                        0x0 => {
                            // Set Vx = Vy.
                            self.v_reg[x] = self.v_reg[y];
                        },
                        0x1 => {
                            // Set Vx = Vx OR Vy.
                            self.v_reg[x] = self.v_reg[x] | self.v_reg[y];
                        },
                        0x2 => {
                            // Set Vx = Vx AND Vy.
                            self.v_reg[x] = self.v_reg[x] & self.v_reg[y];
                        },  
                        0x3 => {
                            // Set Vx = Vx XOR Vy.
                            self.v_reg[x] = self.v_reg[x] ^ self.v_reg[y];
                        },
                        0x4 => {
                            // Set Vx = Vx + Vy, set VF = carry 
                            let (sum, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                            self.v_reg[x] = sum;
                            self.v_reg[0xF] = if carry {1} else {0};
                        },
                        0x5 => {
                            // Set Vx = Vx - Vy, set VF = NOT borrow.
                            let (sub, carry) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                            self.v_reg[x] = sub;
                            self.v_reg[0xF] = if carry {0} else {1};
                        },
                        0x6 => {
                            // Set Vx = Vx SHR 1, effectively divides by 2
                            let carry = self.v_reg[x] & 1;
                            self.v_reg[x] >>= 1;
                            self.v_reg[0xF] = carry;
                        },
                        0x7 => {
                            // Set Vx = Vy - Vx, set VF = NOT borrow.
                            let (sub, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                            self.v_reg[x] = sub;
                            self.v_reg[0xF] = if carry {0} else {1};
                        },
                        0xE => {
                            // Set Vx = Vx SHL 1, effectively multiplies by 2
                            let carry = (self.v_reg[x] & 0x80) >> 7;
                            self.v_reg[x] <<= 1;
                            self.v_reg[0xF] = carry;
                        },
                        _ => {},
                    }
                },
                0xC => {
                    // Set Vx = random byte AND nn
                    let mut rng = thread_rng();
                    let random_byte: u8 = rng.gen();
                    self.v_reg[opcode.x as usize] = random_byte & opcode.nn;
                },
                0xF => {
                    if opcode.nn == 0x07 {self.v_reg[opcode.x as usize] = self.delay_timer}
                }
                _ => {println!("Unimplemented opcode: {:04X}", opcode.opcode)},
            }
            
        }

        fn add_to_register(&mut self, opcode: Opcode) {
            //add value to register VX
            let i: usize = opcode.x.into();
            self.v_reg[i] = self.v_reg[i].wrapping_add(opcode.nn);
        }

        fn set_index_register(&mut self, opcode: Opcode) {
            //sets index register 
            match opcode.a {
                0xA => {self.index = opcode.nnn;}
                0xF => {
                    match opcode.nn {
                        0x1E => {
                            self.index += self.v_reg[opcode.x as usize] as u16;
                        }
                        0x29 => {
                            // Set I = location of sprite for digit Vx
                            // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx
                            let letter = self.v_reg[opcode.x as usize] & 0xF;
                            self.index = (letter as u16) *5;
                        }
                        _ => println!("Unimplemented opcode: {:04X}", opcode.opcode)
                    }
                }
                _ => println!("Unimplemented opcode: {:04X}", opcode.opcode)
            }
        }   
        
        fn draw(&mut self, opcode: Opcode) {
            // sprite pixels in memory are XORed onto the screen

            // first get x and y coordinates
            // I've hard-coded the screen size here, perhaps change this
            let x_coord = self.v_reg[opcode.x as usize] % 64;
            let y_coord = self.v_reg[opcode.y as usize] % 32;

            //set VF to 0
            self.v_reg[0xF] = 0;

            //for n rows (starting at memory address stored in I)
            for row in 0..(opcode.n) {
                // so to access the memory address, we want to use i = index + row
                let i = self.index + row as u16;

                // get the nth byte of sprite data from this address
                let nth_sprite = self.memory.data[i as usize];
                let y = y_coord + row;

                //stop if you reach bottom edge of the screen
                if y >= 32 {break}

                // for each of the 8 pixels in the sprite row, xor them onto the screen
                for bit in 0..8 {
                    let x = x_coord + bit;
                    //if you reach right edge of the screen stop drawing this row (break loop)
                    if x >= 64 {break}
                    let sprite_pix = (nth_sprite >> (7 - bit)) & 1;
                    let display_index = (y as usize)*WIDTH + (x as usize);

                    if sprite_pix == 1 {
                        // turn off screen_pix and set VF to 1
                        if self.display[display_index] {self.v_reg[0xF] = 1}
                        self.display[display_index] ^= true;
                    }
                }
            }
        }
        

        fn wait(&mut self, opcode: Opcode) {
            // Wait for a key press, store the value of the key in Vx

            let found = false;
            // check to see if a key is pressed
            for i in 0..=15 {
                if self.keypad[i] {
                    // if so, store its value in Vx
                    self.v_reg[opcode.x as usize] = i as u8;
                }
            }
            // if not, adjust program counter -2
            if !found {self.program_counter -=2}
        }

        fn set_delay_timer(&mut self, opcode: Opcode) {
            self.delay_timer = self.v_reg[opcode.x as usize];
        }

        fn set_sound_timer(&mut self, opcode: Opcode) {
            self.sound_timer = self.v_reg[opcode.x as usize];
        }

        fn store_bcd_mem(&mut self, opcode: Opcode) {
            // Store BCD representation of Vx in memory locations I, I+1, and I+2
            let value = self.v_reg[opcode.x as usize];
            let hunds = (value / 100) % 10;
            let tens = (value / 10) % 10;// >> 4 & 0x0F;
            let ones = value % 10;//& 0x0F;
            let i = self.index as usize;

            // hundreds digit
            self.memory.write_byte(i,hunds);
            // tens digit
            self.memory.write_byte(i+1,tens);
            // ones digit
            self.memory.write_byte(i+2,ones);
        }

        fn store_mem(&mut self, opcode: Opcode) {
            // Store registers V0 through Vx in memory starting at location I
            let mut i = self.index as usize;
            for j in 0..=opcode.x as usize {
                self.memory.write_byte(i, self.v_reg[j]);
                i += 1;
            }
        }

        fn read_mem(&mut self, opcode: Opcode) {
            // Read registers V0 through Vx from memory starting at location I
            let mut i = self.index as usize;
            for j in 0..=opcode.x as usize {
                //self.memory.write_byte(i, self.v_reg[j]);
                self.v_reg[j] = self.memory.read_byte(i);
                i += 1;
            }

        }

    }



    /* 
    
    TODO:
    o add all opcodes to match statement
    o create functions for all opcodes
    - error handling
    o hard-coded screen size
    o get IBM logo working
    - add in timer to the main loop (probably in app.rs)
    - tidy up draw function (x's and y's)
    - maybe don't pass the actual opcode in to the match statement functions, but a reference to it
    - if i'm passing the opcode in as an argument for everything, maybe make it a field in the chip8 struct? Current opcode?
    - check your v_reg index calls are correct (use 0xF instead of 15 as index?)
    - maybe let x = opcode.x as usize at the beg of set function
    - check bitwise ops 


    for IBM logo: 

    00E0 (clear screen) TICK
    1NNN (jump)
    6XNN (set register VX)
    7XNN (add value to register VX)
    ANNN (set index register I)
    DXYN (display/draw)

    ALL OPCODES:

        +    00E0 - CLS
        o    00EE - RET
        x    0nnn - SYS addr
        +    1nnn - JP addr
        o    2nnn - CALL addr
        o    3xkk - SE Vx, byte
        o    4xkk - SNE Vx, byte
        o    5xy0 - SE Vx, Vy
        +    6xkk - LD Vx, byte
        +    7xkk - ADD Vx, byte
        o    8xy0 - LD Vx, Vy
        o    8xy1 - OR Vx, Vy
        o    8xy2 - AND Vx, Vy
        o    8xy3 - XOR Vx, Vy
        o    8xy4 - ADD Vx, Vy
        o    8xy5 - SUB Vx, Vy
        o    8xy6 - SHR Vx {, Vy}
        o    8xy7 - SUBN Vx, Vy
        o    8xyE - SHL Vx {, Vy}
        o    9xy0 - SNE Vx, Vy
        +    Annn - LD I, addr
        o    Bnnn - JP V0, addr
        o    Cxkk - RND Vx, byte
        +    Dxyn - DRW Vx, Vy, nibble
        o    Ex9E - SKP Vx
        o    ExA1 - SKNP Vx
        o    Fx07 - LD Vx, DT
        o    Fx0A - LD Vx, K
        o    Fx15 - LD DT, Vx
        o    Fx18 - LD ST, Vx
        o    Fx1E - ADD I, Vx
        o    Fx29 - LD F, Vx
        o    Fx33 - LD B, Vx
        o    Fx55 - LD [I], Vx
        o    Fx65 - LD Vx, [I]
    
     */