// control center

// I have decided to take the filepath to the program as a command line argument for now
// maybe a simply gui in future

mod chip8;
mod memory;
mod timer;
mod input;
mod sound;
mod app;
mod opcode;

use std::env;
use app::App;
use winit::event_loop::EventLoop;

fn main() {

    println!("in main");

    // we'll read the filepath from the command line arguments
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please provide a filepath as the first command line argument.");

    // create a window and event handling using application handler
    let event_loop = EventLoop::new().expect("Failed to create event loop.");
    let mut app = App::new(file_path);
    event_loop.run_app(&mut app).expect("Failed to run app.");
}