use wowser::font::{BDFFont, FontError};

use std::env;
use std::fs;

fn main() -> Result<(), FontError> {
    let args: Vec<String> = env::args().collect();
    let font_file = args.get(1).expect("Font file not passed in");

    let font_bytes = fs::read(font_file).expect("Unable to read file");
    let font = BDFFont::load(&font_bytes)?;
    println!("Font: {:?}", font);
    Ok(())
}
