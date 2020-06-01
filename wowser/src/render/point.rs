/// Represents a location in a 2D coordinate system
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Creates a new Point offset from the current one
    pub fn offset(&self, dx: f32, dy: f32) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }
}
