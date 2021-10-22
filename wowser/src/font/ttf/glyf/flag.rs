use crate::util::{Bit, BitExtractor};

use super::{FlagCoordinate, FlagCoordinateSize};

#[derive(Debug, Clone)]
pub struct Flag {
    flag: u8,
}

impl Flag {
    pub fn new(flag: u8) -> Self {
        Self { flag }
    }

    #[allow(dead_code)]
    pub fn is_on_curve(&self) -> bool {
        self.flag.get_bit(Bit::Seven)
    }

    pub fn get_x_coordinate_size(&self) -> FlagCoordinateSize {
        if self.flag.get_bit(Bit::Six) {
            FlagCoordinateSize::U8
        } else {
            FlagCoordinateSize::U16
        }
    }

    pub fn get_y_coordinate_size(&self) -> FlagCoordinateSize {
        if self.flag.get_bit(Bit::Five) {
            FlagCoordinateSize::U8
        } else {
            FlagCoordinateSize::U16
        }
    }

    pub fn repeats(&self) -> bool {
        self.flag.get_bit(Bit::Four)
    }

    pub fn get_x_coordinate_setting(&self) -> FlagCoordinate {
        Self::get_coordinate_setting(self.get_x_coordinate_size(), self.flag.get_bit(Bit::Three))
    }

    pub fn get_y_coordinate_setting(&self) -> FlagCoordinate {
        Self::get_coordinate_setting(self.get_y_coordinate_size(), self.flag.get_bit(Bit::Two))
    }

    #[allow(dead_code)]
    pub fn may_contours_overlap(&self) -> bool {
        self.flag.get_bit(Bit::One)
    }

    fn get_coordinate_setting(
        coordinate_size: FlagCoordinateSize,
        coordinate_bit: bool,
    ) -> FlagCoordinate {
        match (coordinate_size, coordinate_bit) {
            (FlagCoordinateSize::U8, true) => FlagCoordinate::PositiveU8,
            (FlagCoordinateSize::U8, false) => FlagCoordinate::NegativeU8,
            (FlagCoordinateSize::U16, true) => FlagCoordinate::CopyPrevious,
            (FlagCoordinateSize::U16, false) => FlagCoordinate::DeltaI16,
        }
    }
}
