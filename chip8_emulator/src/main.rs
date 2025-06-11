// control center

// I have decided to take the filepath to the program as a command line argument for now
// maybe a simply gui in future

mod chip8;
mod memory;
mod timer;
mod audio_state;
mod opcode;

use std::{env, time::Instant};
use minifb::{Key, Window, WindowOptions, KeyRepeat};
use crate::{audio_state::AudioState, chip8::Chip8};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

const TICK_RATE: f64 = 60.0;
const TICK_DURATION: f64 = 1.0 / TICK_RATE;
const CYCLES_PER_FRAME: usize = 10; // 8-16 to run at 500-1000hz (given the tick rate) -- adjust as necessary for the program you want to run

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

    // timer stuff
    let mut last = Instant::now();
    let mut accumulator: f64 = 0.0;
    let mut audio = AudioState::new();


    while window.is_open() && !window.is_key_down(Key::Escape) {

        // timer stuff
        let now = Instant::now();
        let dur_len = now.duration_since(last);
        last = now;
        accumulator += dur_len.as_secs_f64();

        // update timers if we have a tick
        while accumulator >= TICK_DURATION {
            chip8.update_timers();
            audio.update(chip8.sound_timer.current_time);
            accumulator -= TICK_DURATION;
        }

        for _i in 0..CYCLES_PER_FRAME {
            chip8.run_cycle_once();
        }

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

        window.get_keys_pressed(KeyRepeat::No).iter().for_each(|key|
            match key {
                Key::Key1 => chip8.update_keypad(0x1,true),
                Key::Key2 => chip8.update_keypad(0x2,true),
                Key::Key3 => chip8.update_keypad(0x3,true),
                Key::Key4 => chip8.update_keypad(0xC,true),
                Key::Q => chip8.update_keypad(0x4,true),
                Key::W => chip8.update_keypad(0x5,true),
                Key::E => chip8.update_keypad(0x6,true),
                Key::R => chip8.update_keypad(0xD,true),
                Key::A => chip8.update_keypad(0x7,true),
                Key::S => chip8.update_keypad(0x8,true),
                Key::D => chip8.update_keypad(0x9,true),
                Key::F => chip8.update_keypad(0xE,true),
                Key::Z => chip8.update_keypad(0xA,true),
                Key::X => chip8.update_keypad(0x0,true),
                Key::C => chip8.update_keypad(0xB,true),
                Key::V => chip8.update_keypad(0xF,true),
                _ => (),
            }
        );

        window.get_keys_released().iter().for_each(|key|
            match key {
                Key::Key1 => chip8.update_keypad(0x1,false),
                Key::Key2 => chip8.update_keypad(0x2,false),
                Key::Key3 => chip8.update_keypad(0x3,false),
                Key::Key4 => chip8.update_keypad(0xC,false),
                Key::Q => chip8.update_keypad(0x4,false),
                Key::W => chip8.update_keypad(0x5,false),
                Key::E => chip8.update_keypad(0x6,false),
                Key::R => chip8.update_keypad(0xD,false),
                Key::A => chip8.update_keypad(0x7,false),
                Key::S => chip8.update_keypad(0x8,false),
                Key::D => chip8.update_keypad(0x9,false),
                Key::F => chip8.update_keypad(0xE,false),
                Key::Z => chip8.update_keypad(0xA,false),
                Key::X => chip8.update_keypad(0x0,false),
                Key::C => chip8.update_keypad(0xB,false),
                Key::V => chip8.update_keypad(0xF,false),
                _ => (),
            }
        );
    }
}