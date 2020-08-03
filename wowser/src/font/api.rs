use crate::util::Point;
use std::borrow::Cow;

pub trait Font {
    fn render_character(&self, character: char) -> Option<RenderedCharacter<'_>>;
}

#[derive(Debug)]
pub struct RenderedCharacter<'a> {
    pub bitmap: Cow<'a, [u8]>,
    pub width: u32,
    pub offset: Point<i32>,
    pub next_char_offset: u32,
}
