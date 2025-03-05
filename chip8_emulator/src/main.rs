// control center

// I have decided to take the filepath to the program as a command line argument for now
// maybe a simply gui in future

mod chip8;
mod memory;
//mod timer;
//mod sound;
//mod app;
mod opcode;

use std::env;
//use app::App;
//use winit::event_loop::EventLoop;

use minifb::{Key, Window, WindowOptions};
use crate::chip8::Chip8;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

//#[tokio::main]

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Please provide a filepath as the first command line argument.");

    let mut chip8 = Chip8::new();

    chip8.load_program(file_path).expect("Failed to load program.");

    let mut window = Window::new(
        "CHIP-8 Emulator",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let mut buffer = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.run_cycle_once();

        // Update the pixel buffer
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if chip8.display[y * WIDTH + x] { 0xFFFFFF } else { 0x000000 };
                buffer[y * WIDTH + x] = color;
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update buffer");

        
        // use chip8.update_keypad()
    }

}
 
/*

TO DO: 
- add in keypad stuff
- add in proper error handling

*/