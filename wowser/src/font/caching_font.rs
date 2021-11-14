use std::collections::HashMap;

use super::{Font, RenderedCharacter};

#[derive(Hash, PartialEq, Eq)]
struct CacheKey {
    character: char,
    point_size: u32,
}

impl CacheKey {
    fn new(character: char, point_size: f32) -> Self {
        Self {
            character,
            point_size: point_size.to_bits(),
        }
    }
}

pub struct CachingFont {
    font: Box<dyn Font>,
    character_map: HashMap<CacheKey, Option<RenderedCharacter>>,
}

impl CachingFont {
    pub fn wrap(font: Box<dyn Font>) -> CachingFont {
        CachingFont {
            font,
            character_map: HashMap::new(),
        }
    }

    pub fn render_character(
        &mut self,
        character: char,
        point_size: f32,
    ) -> Option<RenderedCharacter> {
        let font = &self.font;
        self.character_map
            .entry(CacheKey::new(character, point_size))
            .or_insert_with(|| font.render_character(character, point_size))
            .clone()
    }
}
