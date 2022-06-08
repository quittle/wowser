use super::json_number::JsonNumber;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(JsonNumber),
    Boolean(bool),
    Null,
}
