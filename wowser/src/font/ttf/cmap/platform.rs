pub enum Platform {
    Unicode,
    Macintosh,
    Iso,
    Windows,
    Custom,
    UserDefined(u16),
    Unknown(u16),
}

impl Platform {
    pub fn new(platform_id: u16) -> Platform {
        match platform_id {
            0 => Self::Unicode,
            1 => Self::Macintosh,
            2 => Self::Iso,
            3 => Self::Windows,
            4 => Self::Custom,
            240..=255 => Self::UserDefined(platform_id),
            _ => Self::Unknown(platform_id),
        }
    }
}
