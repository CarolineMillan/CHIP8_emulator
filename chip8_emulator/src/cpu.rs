// chip8 struct 
// implements fetch - decode - execute cycle using opcode
// opcode functions are in here, not in opcode.rs

mod display.rs;
mod memory.rs;
mod opcode.rs;
mod timer.rs;
mod input.rs;
mod sound.rs;

struct Chip8 {
    memory: Memory,         // 4KB RAM, fontset, etc.
    display: Display,       // 64x32 screen
    program_counter: u16,                // Program Counter (0x200-0xFFF)
    index: u16,             // Index Register
    stack: [u16; 16],       // 16-level call stack
    stack_pointer: u8,                 // Stack Pointer
    v_reg: [u8; 16],            // Variable registers V0-VF
    delay_timer: Timer,     // Countdown timer
    sound_timer: Timer,     // Beep timer
    keypad: [bool; 16],     // State of the 16 CHIP-8 keys
    }

    impl Chip8 {
        fn new() -> Self {

    }
