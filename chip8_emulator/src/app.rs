// app to create window using winit

use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState}, //KeyboardInput, VirtualKeyCode},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};
use winit::keyboard::KeyCode;
use winit::event::WindowEvent::KeyboardInput;
//use winit::event::VirtualKeyCode;
use winit::event::KeyEvent;
use winit::keyboard::PhysicalKey;
use winit::keyboard::Key;
//use winit::event::VirtualKeyCode;

use crate::chip8::Chip8;

use pixels::{Pixels, SurfaceTexture};


#[derive(Debug)]
pub struct App {
    pub chip8: Chip8,
    pub window: Option<Window>,
}

impl App {
    pub fn new(program_fp: &str) -> Self {

        let mut chip8 = Chip8::new();
        chip8.load_program(program_fp).expect("Failed to load program.");

        println!("in app, new");
        
        Self {chip8, window: None}
    }
    // A helper function that creates the Pixels instance on demand.
    fn create_pixels(&self) -> Pixels {
        let window = self.window.as_ref().expect("No window available");
        let surface_texture = SurfaceTexture::new(64, 32, window);
        Pixels::new(64, 32, surface_texture)
            .expect("Failed to create pixel buffer")
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create window and display

        if self.window.is_some() {
            println!("Window already exists!");
         }
        else {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {

        

        match event {
            WindowEvent::CloseRequested => {
                // Print to console and close the application when the window is closed
                println!("Window closed.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {

                // if no window: 
                if !self.window.is_some() {
                    self.resumed(event_loop);
                }
                
                // Run one cycle of the CHIP-8 emulator
                // this decodes and executes one opcode
                self.chip8.run_cycle_once();

                // if there's a new display then render it

                let mut pixels = self.create_pixels();

                // Access the pixel data and set the color
                let frame = pixels.frame_mut();

                let mut x = 0 as usize;
                let mut y = 0 as usize;

                for col in self.chip8.display {
                    for pix in col {
                        let index = (y*64+x) as usize*4;

                        if pix {
                            frame[index] = 0xFF;     // Red
                            frame[index + 1] = 0xFF; // Green
                            frame[index + 2] = 0xFF; // Blue
                            frame[index + 3] = 0xFF; // Alpha (fully opaque)
                        }
                        else {
                            frame[index] = 0x00;     // Red
                            frame[index + 1] = 0x00; // Green
                            frame[index + 2] = 0x00; // Blue
                            frame[index + 3] = 0xFF; // Alpha (still fully opaque)
                        }

                        y += 1;
                    }
                    x += 1;
                    y = 0;
                }

                // Then render:
                pixels.render().expect("Render failed");

                // Borrow the window to request a redraw.
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
            }

            //use winit::event::{WindowEvent, ElementState, VirtualKeyCode};
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {

                // Destructure the 'event' field
                let KeyEvent { state, physical_key , logical_key, .. } = event;

                // Set value as true if pressed, false if released
                let value = state == ElementState::Pressed;
                let mut key = 0x0;

                // match logical_key to chip-8 keypad
                match physical_key {
                    PhysicalKey::Code(KeyCode::Digit1) => key = 0x1,
                    PhysicalKey::Code(KeyCode::Digit2) => key = 0x2,
                    PhysicalKey::Code(KeyCode::Digit3) => key = 0x3,
                    PhysicalKey::Code(KeyCode::Digit4) => key = 0xC,
                    PhysicalKey::Code(KeyCode::KeyQ) => key = 0x4,
                    PhysicalKey::Code(KeyCode::KeyW) => key = 0x5,
                    PhysicalKey::Code(KeyCode::KeyE) => key = 0x6,
                    PhysicalKey::Code(KeyCode::KeyR) => key = 0xD,
                    PhysicalKey::Code(KeyCode::KeyA) => key = 0x7,
                    PhysicalKey::Code(KeyCode::KeyS) => key = 0x8,
                    PhysicalKey::Code(KeyCode::KeyD) => key = 0x9,
                    PhysicalKey::Code(KeyCode::KeyF) => key = 0xE,
                    PhysicalKey::Code(KeyCode::KeyZ) => key = 0xA,
                    PhysicalKey::Code(KeyCode::KeyX) => key = 0x0,
                    PhysicalKey::Code(KeyCode::KeyC) => key = 0xB,
                    PhysicalKey::Code(KeyCode::KeyV) => key = 0xF,
                    _ => todo!(),
                };

                // You would now update the keypad in the Chip-8 instance
                self.chip8.update_keypad(key, value);  // You can adjust the key mapping as needed
            }


            /*
            WindowEvent::KeyboardInput { device_id: _, event: _, is_synthetic: _ } => {
                // store in self.keypad -- do I need to?
                // update chip8.keypad

                // find which key has been changed and save this value as "key"
                // do i need a virtual_keycode in order to do this?

                // now look at the element state. Let value = true if pressed, value = false if released

                let KeyboardInput { state, virtual_keycode, .. } = event;

                //this is the state field of the key that's been pressed
                let value = state == ElementState::Pressed;

                //let key = physical_key;
                

                self.chip8.update_keypad(key, value);
            }
            */
            _ => {}
        }
    }
}


/*

TODO:
- fix x's and y's when rendering
- add if statement so you don't render every iteration of the run_cycle loop
- add keypad tracking

*/