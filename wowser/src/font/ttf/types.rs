use std::ops::Add;

/// FUnits, the smallest measurable distance in em space.
pub type FWord = i16;

/// Seconds since midnight, Jan 1, 1904
pub type LongDateTime = i64;

/// <https://www.hugi.scene.org/online/coding/hugi%2015%20-%20cmtadfix.htm>
pub struct F16dot16 {
    pub val: u32,
}

impl Add for F16dot16 {
    type Output = F16dot16;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val + rhs.val,
        }
    }
}

impl From<F16dot16> for f32 {
    fn from(value: F16dot16) -> Self {
        value.val as f32 / (2_u16.pow(16) as f32)
    }
}
