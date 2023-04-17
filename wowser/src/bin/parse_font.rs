use wowser::font::Ttf;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::{get_bit, Bit, Point};
use wowser::{
    font::{BDFFont, Font, FontError},
    render::Color,
};

use std::env;
use std::fs;
use std::{borrow::Borrow, thread};

fn main() -> Result<(), FontError> {
    let args: Vec<String> = env::args().collect();
    let font_file = args.get(1).expect("Font file not provided");
    let message = args.get(2).expect("Message to print not provided");

    let font_bytes = fs::read(font_file).expect("Unable to read file");
    let font: Box<dyn Font> = if font_file.ends_with(".bdf") {
        Box::new(BDFFont::load(&font_bytes)?)
    } else if font_file.ends_with(".ttf") {
        Box::new(Ttf::load(font_bytes)?)
    } else {
        panic!("Unknown font file type");
    };

    let mut point_size = 12.0;
    let characters = message.chars().map(|c| {
        point_size *= 1.2;
        font.render_character(c, point_size)
    });

    // Draw character in GUI
    startup::start();
    {
        let window_rc = Window::new().expect("Unable to make ui.");
        thread::sleep(std::time::Duration::from_millis(1000));
        let mut offset: Point<f32> = Point {
            x: 10_f32,
            y: 10_f32,
        };
        {
            let mut window = window_rc.borrow_mut();
            let mut window_mutator = window.mutate();
            for char in characters.flatten() {
                window_mutator
                    .draw_bitmap(
                        &(offset.borrow() + &char.offset).into(),
                        &char.bitmap,
                        char.width as u32,
                        &Color::RED,
                    )
                    .expect("Unable to draw bitmap");
                offset.x += char.next_char_offset;
            }
        }
        thread::sleep(std::time::Duration::from_millis(200000));
    }
    startup::stop();
    Ok(())
}

#[allow(dead_code)]
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
