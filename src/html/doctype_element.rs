pub enum DoctypeElement<'a> {
    Specified { full_type: &'a str },
    Unspecified,
}
