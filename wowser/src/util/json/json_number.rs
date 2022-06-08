#[derive(Debug, PartialEq)]
pub enum JsonNumber {
    F64(f64),
    I64(i64),
    String(String),
}
