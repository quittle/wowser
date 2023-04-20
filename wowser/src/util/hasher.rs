use std::{collections::hash_map::DefaultHasher, hash::Hasher};

pub trait HasherExt {
    fn write_str<'a>(&'a mut self, str: &str) -> &'a mut Self;
}

impl<H> HasherExt for H
where
    H: Hasher,
{
    fn write_str<'a>(&'a mut self, str: &str) -> &'a mut Self {
        self.write(str.as_bytes());
        self
    }
}

pub trait Hashable {
    fn hash_u16(&self) -> u16;
}

impl Hashable for str {
    #[allow(unstable_name_collisions)]
    fn hash_u16(&self) -> u16 {
        let mut hasher = DefaultHasher::new();
        hasher.write_str(self); // If there ends up being a funciton name collision then we can rip out most of this code :D
        let hash: u64 = hasher.finish();
        hash as u16
    }
}
