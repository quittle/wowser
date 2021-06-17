use std::ops::*;
pub trait Number: Add + Sub + Mul + Div + Neg + PartialOrd + PartialEq + Sized + Copy {}

impl<T> Number for T where T: Add + Sub + Mul + Div + Neg + PartialOrd + PartialEq + Sized + Copy {}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rect<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Number + Add<Output = T>> Rect<T> {
    pub fn offset(&self, x: T, y: T) -> Rect<T> {
        Rect {
            x: self.x + x,
            y: self.y + y,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Point<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<U: Number, T: Number + Add<U, Output = U>> Add<&mut Point<U>> for &Point<T> {
    type Output = Point<U>;

    fn add(self, rhs: &mut Point<U>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, U: Number, T: Number + Add<U, Output = U>> Add<&Point<U>> for &'a mut Point<T> {
    type Output = Point<U>;

    fn add(self: &'a mut Point<T>, rhs: &Point<U>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U: Number, T: Number + Add<U, Output = U>> Add<&Point<U>> for &Point<T> {
    type Output = Point<U>;

    fn add(self, rhs: &Point<U>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U: Number, T: Number + Add<U, Output = U>> Add<Point<U>> for Point<T> {
    type Output = Point<U>;

    fn add(self, rhs: Point<U>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U: Number, T: Number + AddAssign<U>> AddAssign<Point<U>> for Point<T> {
    fn add_assign(&mut self, rhs: Point<U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

macro_rules! point_from {
    ($i: ty, $o: ty) => {
        impl From<Point<$i>> for Point<$o> {
            fn from(point: Point<$i>) -> Point<$o> {
                Point {
                    x: point.x as $o,
                    y: point.y as $o,
                }
            }
        }
    };
}

point_from!(i32, f32);
point_from!(f32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: -4, y: 5 };

        assert_eq!(&a + &b, Point { x: -3, y: 7 });
    }

    #[test]
    fn from() {
        let _p: Point<f32> = Point { x: 1_i32, y: 1_i32 }.into();
        let _p: Point<i32> = Point { x: 1_f32, y: 1_f32 }.into();
    }
}
