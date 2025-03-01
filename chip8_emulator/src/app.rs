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
/*
    fn update_pixels(&self, pixels: Pixels) -> Pixels {
        // I want a method that updates the pixel buffer with self.chip8.display

        pixels
    }
*/
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create window and display

        //println!("in app, resumed");
        if self.window.is_some() {
            println!("Window already exists!");
         }
        else {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap());
        }

        //println!("in app, resumed 2");
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
                self.chip8.run_cycle();

                // if there's a new display then render it

                let mut pixels = self.create_pixels();
                // Use pixels.render() or update the pixel buffer here with self.chip8's display.
                // For example:
                // pixels.get_frame().copy_from_slice(&self.chip8.display);

                // I want a method that updates the pixel buffer with self.display
                //self.update_pixels();

                // Get the index of the pixel in the buffer
                //let index = (y * 64 + x) as usize * 4; // 4 bytes per pixel

                // Access the pixel data and set the color
                let frame = pixels.frame_mut();

                let mut x = 0 as usize;
                let mut y = 0 as usize;

                for col in self.chip8.display {
                    for pix in col {

                        // I've got the x's and y's mixed up -- fix this
                        let index = (x*64+y) as usize*4;

                        //println!("x: {}, y: {}", x,y);

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

                        x += 1;
                    }
                    y += 1;
                    x = 0;
                    println!("y: {}", y);
                }
/*
                // return true or false based on whether the pixel is "on" (non-zero)
                if frame[index] == 0 {
                    // turn on the pixel
                    frame[index] = 1;
                    return false
                }
                else {
                    // turn off the pixel
                    frame[index] = 0;
                    return true
                }

*/

                // Then render:
                pixels.render().expect("Render failed");

                //println!("in app, redraw requested: {:?}", self.chip8);
               
                // Request another redraw so the emulator keeps running
                //self.window.request_redraw();
                //println!("in app, redraw requested 2");

                // Borrow the window to request a redraw.
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
                //println!("in app, redraw requested 3");
            }
            _ => {}
        }
    }
}