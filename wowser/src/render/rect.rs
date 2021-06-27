#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    const RECT: Rect = Rect {
        x: 1_f32,
        y: 2_f32,
        width: 3_f32,
        height: 4_f32,
    };

    #[test]
    fn rect_top() {
        assert_eq!(2_f32, RECT.top());
    }

    #[test]
    fn rect_right() {
        assert_eq!(4_f32, RECT.right());
    }

    #[test]
    fn rect_bottom() {
        assert_eq!(6_f32, RECT.bottom());
    }

    #[test]
    fn rect_left() {
        assert_eq!(1_f32, RECT.left());
    }

    #[test]
    fn rect_area() {
        assert_eq!(12_f32, RECT.area());
        assert_eq!(
            0_f32,
            Rect {
                x: 0_f32,
                y: 0_f32,
                width: 0_f32,
                height: 1_f32
            }
            .area()
        );
        assert_eq!(
            0_f32,
            Rect {
                x: 0_f32,
                y: 0_f32,
                width: 1_f32,
                height: 0_f32
            }
            .area()
        );
        assert_eq!(
            1_f32,
            Rect {
                x: 0_f32,
                y: 0_f32,
                width: 1_f32,
                height: 1_f32
            }
            .area()
        );
    }
}
