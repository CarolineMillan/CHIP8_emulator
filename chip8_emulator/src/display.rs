// renders display (minifb or SDL2 -- choose)
// stores pixel screen state

/*

Display is a pixel buffer that can render on a given window.

Event handling is done in main.rs
The window is created in main.rs

*/

use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Display<'a> {
    pixels: Pixels<'a>, // a pixel buffer
}

impl<'a> Display<'a> {
    pub fn new(window: Window) -> Self {

        // create surface texture
        let surface_texture = SurfaceTexture::new(640, 320, window);
        
        //create a screen (pixel buffer)
        let pixels = Pixels::new(640, 320, surface_texture)
            .expect("Failed to create pixel buffer");

        //create an instance of Display
        Self {
            pixels,
        }
    }

    pub fn clear_display(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // set all pixels in the display to 0

        // Clear the pixel buffer
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x00; // B
            pixel[3] = 0xff; // A
        }

        // Draw it to the `SurfaceTexture`
        self.pixels.render()?;

        Ok(())

    }
}

// getter method for screen if you want to access it elsewhere