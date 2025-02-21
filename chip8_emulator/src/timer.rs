// delay timer and sound timer 

pub struct Timer {
    current_time: u8,
}

impl Timer {
    pub fn new(start_time: u8) -> Self {
        Self {
            current_time: start_time,
        }
    }
    
    pub fn tick(&mut self) {
        if self.current_time > 0 {
            self.current_time -= 1;
        }
    }
}