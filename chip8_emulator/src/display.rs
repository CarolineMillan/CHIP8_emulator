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
        let surface_texture = SurfaceTexture::new(64, 32, window);
        
        //create a screen (pixel buffer)
        let pixels = Pixels::new(64, 32, surface_texture)
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
    /*
    pub fn update_pixel(&mut self, x: u32, y: u32, colour: [u8;4]) {
            // Get the index of the pixel in the buffer
            let index = (y * 640 + x) as usize * 4; // 4 bytes per pixel

            // Access the pixel data and set the color
            let frame = self.pixels.frame_mut();

            // Set the pixel's RGBA values (4 bytes per pixel)
            frame[index] = colour[0];  // Red
            frame[index + 1] = colour[1]; // Green
            frame[index + 2] = colour[2]; // Blue
            frame[index + 3] = colour[3]; // Alpha
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> [u8; 4] {
        // return colour of the pixel at coord (x,y)

        let index = (y*640 + x) as usize * 4;

        let frame = self.pixels.frame_mut();

        [frame[index], frame[index+1], frame[index+2], frame[index+3]]
    }
    */

    pub fn bitwise_and(&mut self, x: u16, y: u16) -> bool{
        // changes a single pixel based on bitwise and (&) with sprite_pixel = 1
        // returns true or false so that chip8 can adjust VF if necessary

        // Get the index of the pixel in the buffer
        let index = (y * 64 + x) as usize * 4; // 4 bytes per pixel

        // Access the pixel data and set the color
        let frame = self.pixels.frame_mut();

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
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // renders the current pixel buffer onto the window

        self.pixels.render()?;

        Ok(())
    }


}

// getter method for screen if you want to access it elsewhere