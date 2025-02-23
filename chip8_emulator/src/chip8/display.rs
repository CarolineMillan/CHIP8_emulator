// renders display (minifb or SDL2 -- choose)
// stores pixel screen state

pub struct Display {
    screen: [u8; 100],
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: [0; 100],
        }
    }
}

pub fn test() {
    println!("Display module loaded!");
}