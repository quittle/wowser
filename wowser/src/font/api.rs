use crate::util::Point;

pub trait Font {
    fn render_character(&self, character: char) -> Option<RenderedCharacter>;
}

#[derive(Debug, Clone)]
pub struct RenderedCharacter {
    pub bitmap: Vec<u8>,
    pub width: f32,
    pub offset: Point<f32>,
    pub next_char_offset: f32,
}
