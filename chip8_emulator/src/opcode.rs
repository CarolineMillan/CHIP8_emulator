// create a struct for opcodes, so that the nibbles are easily accessible 

// check your use of bitwise operators

pub struct Opcode {
    pub opcode: u16,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl Opcode {

    pub fn new(first_byte: u8, second_byte: u8) -> Self{
        // split up the opcode
        let opcode = u16::from_be_bytes([first_byte, second_byte]);

        //let nibble1 = first_byte >> 4 & 0xF;
        let nibble2 = first_byte & 0xF; //bitwise and with 0xF makes sure the first nibble is 0xF
        let nibble3 = second_byte >> 4 & 0xF;
        let nibble4 = second_byte & 0xF;

        // make sure you know what's going on here
        Self {
            opcode, //: u16::from_be_bytes([first_byte, second_byte]),
            x: nibble2, //second nibble
            y: nibble3, // third nibble
            n: nibble4, //fourth nibble
            nn: second_byte, // third and fourth nibbles
            nnn: (u16::from(nibble2) << 8) | u16::from(second_byte), // second, third and fourth nibbles
        }
    }

}