// delay timer and sound timer 

#[derive(Debug)]
pub struct Timer {
    pub current_time: u8,
}

impl Timer {
    pub fn new(start_time: u8) -> Self {
        
        let t = Self {
            current_time: start_time,
        };
        return t;
    }
    
    pub fn tick(&mut self) {
        if self.current_time > 0 {
            self.current_time = self.current_time.saturating_sub(1);
        }
    }
}