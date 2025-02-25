// renders display (minifb or SDL2 -- choose)
// stores pixel screen state

/*

Display is a pixel buffer that can render on a given window.

Event handling is done in main.rs
The window is created in main.rs

*/

use pixels::{Pixels, SurfaceTexture};
//use winit::dpi::LogicalSize;
use winit::window::{Window, WindowBuilder};

pub struct Display<'a> {
    screen: Pixels<'a>, // a pixel buffer
}

impl<'a> Display<'a> {
    pub fn new(window: Window) -> Self {

        // create surface texture
        let surface_texture = SurfaceTexture::new(640, 320, window);
        
        //create a screen (pixel buffer)
        let screen = Pixels::new(640, 320, surface_texture)
            .expect("Failed to create pixel buffer");

        //create an instance of Display
        Self {
            screen,
        }
    }
}

// getter method for screen if you want to access it elsewhere