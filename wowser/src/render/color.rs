#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}

impl From<&ColorPercent> for Color {
    fn from(color: &ColorPercent) -> Color {
        Color {
            r: (color.r * 255_f32).round() as u8,
            g: (color.g * 255_f32).round() as u8,
            b: (color.b * 255_f32).round() as u8,
            a: (color.a * 255_f32).round() as u8,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct ColorPercent {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<&Color> for ColorPercent {
    fn from(color: &Color) -> ColorPercent {
        ColorPercent {
            r: color.r as f32 / 255_f32,
            g: color.g as f32 / 255_f32,
            b: color.b as f32 / 255_f32,
            a: color.a as f32 / 255_f32,
        }
    }
}
