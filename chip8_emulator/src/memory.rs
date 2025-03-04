// RAM and stack handling (look up fontset)

#[derive(Debug)]
pub struct Memory {
    pub data: [u8; 4056], // 4Kb of memory
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory {
            data: [0; 4056], // initialise with zeros
        };
        mem.load_fonts(); // make sure fonts are always loaded
        mem
    }

    // this is the default font -- you can look into other fonts later if you'd like
    fn load_fonts(&mut self) {
        const CHIP8_FONT: [u8;80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        // apparently this is the convention, no reason not to put it somewhere else below 0x200
        self.data[0x050..0x050+80].copy_from_slice(&CHIP8_FONT);

    }

    //read byte
    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }

    //write byte
    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.data[address] = byte;
    }
    
    //load program
    pub fn load_program(&mut self, program: &[u8]) {

        //we want to start at 0x200, everything above this is usable memory
        let start_pt = 0x200;
        self.data[start_pt..(start_pt+program.len())].copy_from_slice(program);
    }
}