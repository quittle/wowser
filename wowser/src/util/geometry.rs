use std::ops::*;
pub trait Number: Add + Sub + Mul + Div + Neg + PartialOrd + PartialEq + Sized {}

impl<T> Number for T where T: Add + Sub + Mul + Div + Neg + PartialOrd + PartialEq + Sized {}

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
