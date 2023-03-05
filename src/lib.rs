const MOV: u8 = 0b00100010;

fn register_decode(reg: u8, w: bool) -> &'static str {
    match (reg, w) {
        (0b00000000, false) => "al",
        (0b00000001, false) => "cl",
        (0b00000010, false) => "dl",
        (0b00000011, false) => "bl",
        (0b00000100, false) => "ah",
        (0b00000101, false) => "ch",
        (0b00000110, false) => "dh",
        (0b00000111, false) => "bh",

        (0b00000000, true) => "ax",
        (0b00000001, true) => "cx",
        (0b00000010, true) => "dx",
        (0b00000011, true) => "bx",
        (0b00000100, true) => "sp",
        (0b00000101, true) => "bp",
        (0b00000110, true) => "si",
        (0b00000111, true) => "di",
        _ => panic!("Register not recognized !")
    }
}


pub fn decode(bytes: &[u8], output: &mut String) {
    for chunk in bytes.chunks_exact(2) {
        let first = chunk[0];
        let second = chunk[1];

        let opcode = first >> 2;
        let d = ((first & 0b00000010) >> 1) == 1;
        let w = (first & 0b00000001) == 1;
        let (instruction, (mut reg1, mut reg2)) = match opcode {
            MOV => {
                ("mov",
                 {
                     assert_eq!(second & 0b11000000, 0b11000000);


                     let reg1 = (second & 0b00111000) >> 3;
                     let reg2 = second & 0b00000111;

                     (register_decode(reg1, w), register_decode(reg2, w))
                 }
                )
            }
            _ => {
                panic!("Instruction not recognized")
            }
        };
        if !d {
            (reg1, reg2) = (reg2, reg1)
        }
        output.push_str(format!("{instruction} {reg1} {reg2}\n").as_str());
    }
}