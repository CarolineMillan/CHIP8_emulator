// app to create window using winit

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::chip8::Chip8;
#[derive(Debug)]
pub struct App<'a> {
    pub chip8: Option<Chip8<'a>>,
    pub window: Option<Window>,
    program_fp: &'a str,
}

impl<'a> App<'a> {
    pub fn new(program_fp: &'a str) -> Self {
        //let mut chip8 = Chip8::new();
        //chip8.load_program(program_fp).unwrap();
        println!("in app, new");
        Self { chip8: None, window: None, program_fp }
        //println!("in app, new 2: {}", self.chip8);
    }
}

impl<'a> ApplicationHandler for App<'a> 
//where 
//    for<'s> &'s Self: Into<&'a Self>,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        println!("in app, resumed");
        if self.window.is_some() {
            println!("Window already exists!");
         }
        else {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap());
        }

        //self.chip8 = Chip8::new(self.program_fp, self.window);

        println!("in app, resumed 2");
        

        if let Some(ref window) = self.window {
            println!("in app, resumed 4");
            let chip8 = Chip8::new(window);
            self.chip8 = Some(chip8);
            //self.chip8.as_mut().expect("REASON").load_program(self.program_fp).expect("Can't load program into no chip8!");
            
        }

        println!("in app, resumed 3");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {

        

        match event {
            WindowEvent::CloseRequested => {
                // Print to console and close the application when the window is closed
                println!("Window closed.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.resumed(event_loop);
                // Run one cycle of the CHIP-8 emulator
                println!("in app, redraw requested: {:?}", self.chip8);
                if let Some(ref mut chip8)= self.chip8 {
                    println!("in app, redraw requested 4");
                    chip8.run_cycle();
                }
                // Request another redraw so the emulator keeps running
                //self.window.request_redraw();
                println!("in app, redraw requested 2");

                // Borrow the window to request a redraw.
                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
                println!("in app, redraw requested 3");
            }
            _ => {}
        }
    }
}


/*
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub chip8: Chip8,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("A Fantastic Window");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                // Draw.
                // Queue a RedrawRequested event.
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.


                // Run one cycle of the CHIP-8 emulator
                self.chip8.run_cycle();
                // Request another redraw so the emulator keeps running
                self.window.request_redraw();

                //self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

    */
