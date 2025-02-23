// control center

// I have decided to take the filepath to the program as a command line argument for now
// maybe a simply gui in future

mod chip8;

use std::env;
use std::process;

fn main() {
    // we'll read the filepath from the command line arguments

    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Please provide a filepath as the first command line argument.");


    // create a new chip8
    let mut chip8 = chip8::Chip8::new();

    //load program
    if let Err(e) = chip8.load_program(file_path) {
        eprintln!("Error loading program: {}", e);
        process::exit(1);
    }

    // start it running
    chip8.run_cycle();

    // remember to handle errors
}