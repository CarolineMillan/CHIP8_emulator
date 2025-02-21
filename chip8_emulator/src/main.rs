// control center

mod chip8;

fn main() {
    // create a new chip8
    let mut chip8 = chip8::Chip8::new()
    
    // start it running
    chip8.run_cycle()

    // remember to handle errors
}
