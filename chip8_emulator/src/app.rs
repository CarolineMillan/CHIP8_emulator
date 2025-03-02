// app to create window using winit

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

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
                self.resumed(event_loop);
                
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
                    println!("in display for loop");
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
            _ => {}
        }
    }
}


/*

TODO:
- fix x's and y's when rendering
- add if statement so you don't render every iteration of the run_cycle loop

*/