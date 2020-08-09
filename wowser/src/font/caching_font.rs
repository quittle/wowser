use std::collections::HashMap;

use super::{Font, RenderedCharacter};

pub struct CachingFont {
    font: Box<dyn Font>,
    character_map: HashMap<char, Option<RenderedCharacter>>,
}

impl CachingFont {
    pub fn wrap(font: Box<dyn Font>) -> CachingFont {
        CachingFont { font, character_map: HashMap::new() }
    }

    pub fn render_character(&mut self, character: char) -> Option<RenderedCharacter> {
        let font = &self.font;
        self.character_map
            .entry(character)
            .or_insert_with(|| font.render_character(character))
            .clone()
    }
}
