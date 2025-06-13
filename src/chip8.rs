// chip8 struct 
// implements fetch - decode - execute cycle using opcode

use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::timer::Timer;
use crate::TICK_DURATION;


use std::fs;
use std::io;
use std::time::Instant;
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
        pub delay_timer: Timer,     // Countdown timer
        pub sound_timer: Timer,     // Beep timer
        keypad: [bool; 16],     // State of the 16 CHIP-8 keys
        temp_key: Option<u8>,           // stores the i of the key that has been pressed in wait method, once it's released set to 0  
        prev_draw_instant: Instant,
    }

    impl Chip8 {
        pub fn new() -> Self {
            Chip8 {
                memory: Memory::new(),         // 4KB RAM, fontset, etc.
                display: [false; WIDTH*HEIGHT],    // 64x32 screen
                program_counter: 0x200,        // Program Counter (0x200-0xFFF)
                index: 0,                      // Index Register
                stack: [0; 16],                // 16-level call stack
                stack_pointer: 0,              // Stack Pointer
                v_reg: [0; 16],                // Variable registers V0-VF
                delay_timer: Timer::new(60),     // Countdown timer
                sound_timer: Timer::new(60),     // Beep timer
                keypad: [false; 16],           // State of the 16 CHIP-8 keys
                temp_key: None,
                prev_draw_instant: Instant::now(),
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
        }

        pub fn run_cycle_once(&mut self) {
            let current_opcode = self.fetch();
            self.decode_execute(current_opcode);
        }

        pub fn update_timers(&mut self) {
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
            if opcode.a == 0x1 {
                self.program_counter = opcode.nnn as u16;
            }
            else if opcode.a == 0xB {
                self.program_counter = (opcode.nnn as u16) + (self.v_reg[0] as u16);
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
                            self.v_reg[0xF] = 0;
                        },
                        0x2 => {
                            // Set Vx = Vx AND Vy.
                            self.v_reg[x] = self.v_reg[x] & self.v_reg[y];
                            self.v_reg[0xF] = 0;
                        },  
                        0x3 => {
                            // Set Vx = Vx XOR Vy.
                            self.v_reg[x] = self.v_reg[x] ^ self.v_reg[y];
                            self.v_reg[0xF] = 0;
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
                            // OG CHIP-8 version 
                            let carry = self.v_reg[y] & 1;
                            self.v_reg[x] = self.v_reg[y];
                            self.v_reg[x] >>= 1;
                            self.v_reg[0xF] = carry;
                            /*
                            // this is the CHIP-48 / SUPER CHIP version
                            // maybe toggle so you can choose?
                            // Set Vx = Vx SHR 1, effectively divides by 2
                            let carry = self.v_reg[x] & 1;
                            self.v_reg[x] >>= 1;
                            self.v_reg[0xF] = carry;
                            */
                        },
                        0x7 => {
                            // Set Vx = Vy - Vx, set VF = NOT borrow.
                            let (sub, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                            self.v_reg[x] = sub;
                            self.v_reg[0xF] = if carry {0} else {1};
                        },
                        0xE => {
                            // OG CHIP-8 version 
                            let carry = (self.v_reg[y] & 0x80) >> 7;
                            self.v_reg[x] = self.v_reg[y];
                            self.v_reg[x] <<= 1;
                            self.v_reg[0xF] = carry;
                            /*
                            // this is the CHIP-48 / SUPER CHIP version
                            // maybe toggle so you can choose?
                            // Set Vx = Vx SHL 1, effectively multiplies by 2
                            let carry = (self.v_reg[x] & 0x80) >> 7;
                            self.v_reg[x] <<= 1;
                            self.v_reg[0xF] = carry;
                            */
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
                    if opcode.nn == 0x07 {self.v_reg[opcode.x as usize] = self.delay_timer.current_time}
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

            // check the timer to see if we are waiting for a frame refresh
            if (Instant::now() - self.prev_draw_instant).as_secs_f64() < TICK_DURATION {
                self.program_counter -=2;
                return;
            }

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

                // get the ith byte of sprite data from this address
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
            self.prev_draw_instant = Instant::now();
        }
        

        fn wait(&mut self, opcode: Opcode) {
            // Wait for a key press, store the value of the key in Vx
            let mut found = false;
            // check to see if a key is pressed
            if let Some(pressed) = self.temp_key {
                // need to check whether it's been released yet
                if !self.keypad[pressed as usize] {
                    self.v_reg[opcode.x as usize] = pressed;
                    self.temp_key = None;
                    found = true;
                }
            }
            else {
                for i in 0..=15 {
                    if self.keypad[i] {
                        // if so, store its value in Vx
                        self.temp_key = Some(i as u8);
                    }
                }
            }
            // if not, adjust program counter -2
            if !found {self.program_counter -=2}
        }

        fn set_delay_timer(&mut self, opcode: Opcode) {
            self.delay_timer.current_time = self.v_reg[opcode.x as usize];
        }

        fn set_sound_timer(&mut self, opcode: Opcode) {
            self.sound_timer.current_time = self.v_reg[opcode.x as usize];
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
                self.index += 1;
            }
            
        }

        fn read_mem(&mut self, opcode: Opcode) {
            // Read registers V0 through Vx from memory starting at location I
            let mut i = self.index as usize;
            for j in 0..=opcode.x as usize {
                //self.memory.write_byte(i, self.v_reg[j]);
                self.v_reg[j] = self.memory.read_byte(i);
                i += 1;
                self.index += 1;
            }
        }

    }