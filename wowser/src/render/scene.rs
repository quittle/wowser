use super::{color::Color, rect::Rect};

/// An abstract representation of a scene to draw. This should be renderable onto a 2D canvas.
pub enum SceneNode {
    TextSceneNode(TextSceneNode),
    RectangleSceneNode(RectangleSceneNode),
}

impl SceneNode {
    pub fn bounds(&self) -> &Rect {
        match self {
            Self::TextSceneNode(TextSceneNode { bounds, .. }) => bounds,
            Self::RectangleSceneNode(RectangleSceneNode { bounds, .. }) => bounds,
        }
    }

    pub fn mut_bounds(&mut self) -> &mut Rect {
        match self {
            Self::TextSceneNode(TextSceneNode { bounds, .. }) => bounds,
            Self::RectangleSceneNode(RectangleSceneNode { bounds, .. }) => bounds,
        }
    }
}

pub struct TextSceneNode {
    pub bounds: Rect,
    pub text: String,
    pub font_size: f32,
    pub text_color: Color,
}

pub struct RectangleSceneNode {
    pub bounds: Rect,
    pub fill: Color,
    pub border_color: Color,
    pub border_width: f32,
}
