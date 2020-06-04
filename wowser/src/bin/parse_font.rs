use wowser::font::{BDFFont, FontError};
use wowser::startup;
use wowser::ui::Window;
use wowser::util::{get_bit, Bit};

use std::env;
use std::fs;
use std::thread;

fn main() -> Result<(), FontError> {
    let args: Vec<String> = env::args().collect();
    let font_file = args.get(1).expect("Font file not passed in");

    let font_bytes = fs::read(font_file).expect("Unable to read file");
    let font = BDFFont::load(&font_bytes)?;
    let mut bitmap: Vec<Vec<u8>> = vec![];
    for character in font.characters.expect("") {
        let name = character.name.expect("");
        println!("Char: {}", name);
        if name == "U+0037" {
            // "7"
            bitmap = character.bitmap.expect("").bytes;
            break;
        }
    }

    // Print character in ascii
    for line in &bitmap {
        for byte_str in line.iter().map(|byte| byte_to_bit_char(*byte)) {
            print!("{}", byte_str);
        }
        println!();
    }

    // Draw character in GUI
    startup::start();
    {
        let mut window = Window::new().expect("Unable to make ui.");
        thread::sleep(std::time::Duration::from_millis(1000));
        window.draw_bitmap(&bitmap).expect("Unable to draw bitmap");
        thread::sleep(std::time::Duration::from_millis(200000));
    }
    wowser_glfw::terminate();
    Ok(())
}

fn byte_to_bit_char(byte: u8) -> String {
    let mut s = String::new();
    s.push(bool_to_bit_char(get_bit(byte, Bit::Zero)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::One)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Two)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Three)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Four)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Five)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Six)));
    s.push(bool_to_bit_char(get_bit(byte, Bit::Seven)));
    s
}

fn bool_to_bit_char(bit: bool) -> char {
    if bit {
        '.'
    } else {
        ' '
    }
}
