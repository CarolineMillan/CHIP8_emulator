// control center

// I have decided to take the filepath to the program as a command line argument for now
// maybe a simply gui in future

// event handling happens here

// createa a window here

// create a display here and use this to create a new chip8
// maybe change so that you can create a new chip8 without a dsplay, and run a method chip8.create_display(window) to create the display 
// I'm not sure this would work, because you'd need to initialise chip8 without a display while having a display field. 
// I wonder if there's a way to keep the display field optional

// yes there is


mod chip8;
mod display;
mod memory;
mod timer;
mod input;
mod sound;
mod app;
mod opcode;

use std::env;
use std::process;
use display::Display;
use app::App;

use winit::event_loop::EventLoop;

fn main() {

    // we'll read the filepath from the command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please provide a filepath as the first command line argument.");


    // add error handlind to the following (I've just unwrapped them for now)

    // create a window and event handling using application handler
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    //let _res = event_loop.run_app(&mut app);
    event_loop.run_app(&mut app).unwrap();

    // create a display
    let display = Display::new(app.window.unwrap());

    // create a new chip8
    let mut chip8 = chip8::Chip8::new(display);

    //load program
    if let Err(e) = chip8.load_program(file_path) {
        eprintln!("Error loading program: {}", e);
        process::exit(1);
    }

    // start it running
    chip8.run_cycle();

    // remember to handle errors
}