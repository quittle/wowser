use std::hash::Hasher;

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
