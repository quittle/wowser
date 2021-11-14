pub trait Sqrt {
    fn sqrt(self) -> Option<Self>
    where
        Self: Sized;
}

impl Sqrt for u16 {
    fn sqrt(self) -> Option<Self> {
        let root = (self as f32).sqrt() as Self;
        if root * root == self {
            Some(root)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Sqrt;

    #[test]
    fn test_sqrt() {
        assert_eq!(1_u16.sqrt(), Some(1));
        assert_eq!(2_u16.sqrt(), None);
        assert_eq!(3_u16.sqrt(), None);
        assert_eq!(4_u16.sqrt(), Some(2));
        assert_eq!(36_u16.sqrt(), Some(6));
        assert_eq!(37_u16.sqrt(), None);
        assert_eq!(65025_u16.sqrt(), Some(255));
    }
}
