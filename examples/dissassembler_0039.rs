use rust_computer_enhance_exercices::decode;

fn main() {
    let bytes = include_bytes!("../computer_enhance/perfaware/part1/listing_0039_more_movs");
    let mut output = String::new();

    output.push_str("bits 16\n\n");
    decode(bytes, &mut output);
    println!("{output}");
}