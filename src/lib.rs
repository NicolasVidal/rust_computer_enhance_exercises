use std::cmp::Ordering;
use std::slice::Iter;

const MOV_REG_REG_MEM: (u8, u8) = (0b00100010, 2);
const MOV_IM_REG_MEM: (u8, u8) = (0b01100011, 1);
const MOV_IM_REG: (u8, u8) = (0b00001011, 4);
const MOV_MEM_TO_ACC: (u8, u8) = (0b01010000, 1);
const MOV_ACC_TO_MEM: (u8, u8) = (0b01010001, 1);

fn match_op(data: u8, op_and_displacement: (u8, u8)) -> bool {
    (data >> op_and_displacement.1) == op_and_displacement.0
}

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

fn address_decode(r_m: u8, a_mod: u8, it: &mut Iter<u8>) -> String {
    let address = match r_m {
        0b000 => Some("bx + si"),
        0b001 => Some("bx + di"),
        0b010 => Some("bp + si"),
        0b011 => Some("bp + di"),
        0b100 => Some("si"),
        0b101 => Some("di"),
        0b110 => if a_mod == 0b00 { None } else { Some("bp") },
        0b111 => Some("bx"),
        _ => panic!("Unrecognized r_m : {r_m}")
    };

    let displacement = match (a_mod, address) {
        (0b00, Some(_)) => 0i16,
        (0b00, None) => {
            let displacement_l = *it.next().unwrap();
            let displacement_h = *it.next().unwrap();
            i16::from_le_bytes([displacement_l, displacement_h])
        }
        (0b01, _) => {
            (*it.next().unwrap() as i8) as i16
        }
        (0b10, _) => {
            let displacement_l = *it.next().unwrap();
            let displacement_h = *it.next().unwrap();
            i16::from_le_bytes([displacement_l, displacement_h])
        }
        _ => panic!("unrecognized a_mod: {a_mod}")
    };

    if let Some(address) = address {
        match displacement.cmp(&0i16) {
            Ordering::Less => {
                let displacement = -displacement;
                format!("[{address} - {displacement}]")
            }
            Ordering::Equal => format!("[{address}]"),
            Ordering::Greater => format!("[{address} + {displacement}]"),
        }
    } else {
        format!("[{displacement}]")
    }
}

pub fn decode(bytes: &[u8], output: &mut String) {
    let mut it = bytes.iter();
    while let Some(first) = it.next() {
        match *first {
            x if match_op(x, MOV_REG_REG_MEM) => {
                let second = it.next().unwrap();

                let a_mod = (second & 0b11000000) >> 6;
                match a_mod {
                    0b11 => {
                        let d = ((first & 0b00000010) >> 1) == 1;
                        let w = (first & 0b00000001) == 1;

                        let reg1 = (second & 0b00111000) >> 3;
                        let reg2 = second & 0b00000111;

                        let (mut reg1, mut reg2) = (register_decode(reg1, w), register_decode(reg2, w));
                        if !d {
                            (reg1, reg2) = (reg2, reg1)
                        }
                        output.push_str(format!("mov {reg1}, {reg2}\n").as_str());
                    }
                    _ => {
                        let d = ((first & 0b00000010) >> 1) == 1;
                        let w = (first & 0b00000001) == 1;
                        let reg1 = (second & 0b00111000) >> 3;
                        let reg1 = register_decode(reg1, w);
                        let r_m = second & 0b00000111;
                        let memory = address_decode(r_m, a_mod, &mut it);
                        if d {
                            output.push_str(format!("mov {reg1}, {memory}\n").as_str());
                        } else {
                            output.push_str(format!("mov {memory}, {reg1}\n").as_str());
                        }
                    }
                }
            }
            x if match_op(x, MOV_IM_REG_MEM) => {
                let w = (first & 0b00000001) == 1;

                let second = it.next().unwrap();

                let a_mod = (second & 0b11000000) >> 6;

                match a_mod {
                    0b11 => {
                        let reg2 = second & 0b00000111;

                        let reg2 = register_decode(reg2, w);

                        let data1 = *it.next().unwrap();
                        let mut data2 = 0;
                        if w {
                            data2 = *it.next().unwrap();
                        }

                        let data = i16::from_le_bytes([data1, data2]);

                        if w {
                            output.push_str(format!("mov {reg2}, word {data}\n").as_str());
                        } else {
                            output.push_str(format!("mov {reg2}, byte {data}\n").as_str());
                        }
                    }
                    _ => {
                        let r_m = second & 0b00000111;
                        let memory = address_decode(r_m, a_mod, &mut it);

                        let data1 = *it.next().unwrap();
                        let mut data2 = 0;
                        if w {
                            data2 = *it.next().unwrap();
                        }

                        let data = i16::from_le_bytes([data1, data2]);

                        if w {
                            output.push_str(format!("mov {memory}, word {data}\n").as_str());
                        } else {
                            output.push_str(format!("mov {memory}, byte {data}\n").as_str());
                        }
                    }
                }
            }
            x if match_op(x, MOV_IM_REG) => {
                let w = ((first & 0b00001000) >> 3) == 1;
                let reg = first & 0b00000111;

                let reg = register_decode(reg, w);

                let data1 = *it.next().unwrap();

                let mut data2 = 0;
                if w {
                    data2 = *it.next().unwrap();
                }

                let data = i16::from_le_bytes([data1, data2]);

                output.push_str(format!("mov {reg}, {data}\n").as_str());
            }
            x if match_op(x, MOV_MEM_TO_ACC) => {
                let w = (first & 0b00000001) == 1;
                let data1 = *it.next().unwrap();
                let mut data2 = 0;
                if w {
                    data2 = *it.next().unwrap();
                }

                let data = i16::from_le_bytes([data1, data2]);

                output.push_str(format!("mov ax, [{data}]\n").as_str());
            }

            x if match_op(x, MOV_ACC_TO_MEM) => {
                let w = (first & 0b00000001) == 1;
                let data1 = *it.next().unwrap();
                let mut data2 = 0;
                if w {
                    data2 = *it.next().unwrap();
                }

                let data = i16::from_le_bytes([data1, data2]);

                output.push_str(format!("mov [{data}], ax\n").as_str());
            }
            _x => {
                panic!("Instruction not recognized : {_x}, output was : \n{output}")
            }
        };
    }
}