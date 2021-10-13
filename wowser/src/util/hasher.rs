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
    fn hash_u16(&self) -> u16 {
        let mut hasher = DefaultHasher::new();
        hasher.write_str(self);
        let hash: u64 = hasher.finish();
        hash as u16
    }
}
