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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Point<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

impl<T: Number + Add<Output = T>> Add for &Point<T> {
    type Output = Point<T>;

    fn add(self: Self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: -4, y: 5 };

        assert_eq!(&a + &b, Point { x: -3, y: 7 });
    }
}
