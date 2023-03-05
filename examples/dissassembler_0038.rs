use rust_computer_enhance_exercices::decode;

fn main() {
    let bytes = include_bytes!("../computer_enhance/perfaware/part1/listing_0038_many_register_mov");
    let mut output = String::new();

    output.push_str("bits 16\n\n");
    decode(bytes, &mut output);
    println!("{output}");
}