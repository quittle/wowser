use crate::parse::{Interpreter, Lexer, Parser};

use self::{json_interpreter::JsonInterpreter, json_rule::JsonRule, json_token::JsonToken};

mod json_interpreter;
mod json_number;
mod json_rule;
mod json_token;
mod json_value;

pub use json_number::JsonNumber;
pub use json_value::JsonValue;

pub fn parse_json(value: &str) -> Result<JsonValue, String> {
    if let Some(tokens) = Lexer::new(JsonToken::Document).parse(value) {
        let parser = Parser {};
        let parse_result = parser.parse(&tokens, &JsonRule::Document);
        match parse_result {
            Ok(ast) => {
                let interpreter = JsonInterpreter {};
                if let Some(value) = interpreter.interpret(&ast) {
                    Ok(value)
                } else {
                    Err(String::from("Invalid json"))
                }
            }
            Err(error) => Err(String::from(error)),
        }
    } else {
        Err(String::from("Unable to parse JSON"))
    }
}

#[cfg(test)]
mod tests {
    use crate::util::json::json_number::JsonNumber;

    use super::*;

    fn unable_to_parse_err() -> Result<JsonValue, String> {
        Err(String::from("Unable to parse JSON"))
    }

    #[test]
    fn test_parse_empty_values() {
        assert_eq!(parse_json(""), unable_to_parse_err());
        assert_eq!(parse_json(" "), unable_to_parse_err());
        assert_eq!(parse_json(" \t\n "), unable_to_parse_err());
    }

    #[test]
    fn test_parse_null() {
        assert_eq!(parse_json("null"), Ok(JsonValue::Null));
        assert_eq!(parse_json(" null "), Ok(JsonValue::Null));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_json("true"), Ok(JsonValue::Boolean(true)));
        assert_eq!(parse_json(" false "), Ok(JsonValue::Boolean(false)));
    }

    #[test]
    fn test_number_parse() {
        assert_eq!(parse_json("0"), Ok(JsonValue::Number(JsonNumber::I64(0))));
        assert_eq!(parse_json("-0"), Ok(JsonValue::Number(JsonNumber::I64(-0))));
        assert_eq!(parse_json("-00"), unable_to_parse_err());
        assert_eq!(
            parse_json("123"),
            Ok(JsonValue::Number(JsonNumber::I64(123)))
        );
        assert_eq!(
            parse_json("123.456"),
            Ok(JsonValue::Number(JsonNumber::F64(123.456)))
        );
        assert_eq!(parse_json("123."), unable_to_parse_err());
        assert_eq!(
            parse_json("123.0"),
            Ok(JsonValue::Number(JsonNumber::F64(123.0)))
        );
        assert_eq!(
            parse_json("123.0000"),
            Ok(JsonValue::Number(JsonNumber::F64(123.0)))
        );
        assert_eq!(parse_json("123.-0000"), unable_to_parse_err());
        assert_eq!(
            parse_json("0.0"),
            Ok(JsonValue::Number(JsonNumber::F64(0.0)))
        );
        let absurdly_large_number = "9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
        assert_eq!(
            parse_json(absurdly_large_number),
            Ok(JsonValue::Number(JsonNumber::String(
                absurdly_large_number.into()
            )))
        );
    }

    #[test]
    fn test_string_parse() {
        assert_eq!(parse_json(r#" "" "#), Ok(JsonValue::String("".into())));
        assert_eq!(
            parse_json(r#" "abc" "#),
            Ok(JsonValue::String("abc".into()))
        );
        assert_eq!(
            parse_json(r#" "123" "#),
            Ok(JsonValue::String("123".into()))
        );
        assert_eq!(parse_json(r#" "'" "#), Ok(JsonValue::String("'".into())));
        assert_eq!(
            parse_json(r#" " \" " "#),
            Ok(JsonValue::String(" \" ".into()))
        );
        assert_eq!(
            parse_json(r#""\\n""#),
            Ok(JsonValue::String(r#"\n"#.into())),
            "Order of escaping"
        );
    }

    #[test]
    fn test_array_parse() {
        assert_eq!(parse_json("[]"), Ok(JsonValue::Array(vec![])));
        assert_eq!(
            parse_json("[1]"),
            Ok(JsonValue::Array(vec![JsonValue::Number(JsonNumber::I64(
                1
            ))]))
        );
        assert_eq!(parse_json("[1,]"), unable_to_parse_err());
        assert_eq!(
            parse_json(" [ 1 , false ] "),
            Ok(JsonValue::Array(vec![
                JsonValue::Number(JsonNumber::I64(1)),
                JsonValue::Boolean(false),
            ]))
        );
    }

    #[test]
    fn test_object_parse() {
        assert_eq!(parse_json("{}"), Ok(JsonValue::Object(vec![])));
        assert_eq!(
            parse_json(r#" {"key"} "#),
            Err(String::from("Unable to match any child rules"))
        );
        assert_eq!(
            parse_json(r#" {"key": "value"} "#),
            Ok(JsonValue::Object(vec![(
                String::from("key"),
                JsonValue::String(String::from("value"))
            )]))
        );
        assert_eq!(parse_json(r#" {1: "value"} "#), unable_to_parse_err());
        assert_eq!(
            parse_json(r#" {"a": null, "b": null} "#),
            Ok(JsonValue::Object(vec![
                (String::from("a"), JsonValue::Null),
                (String::from("b"), JsonValue::Null)
            ]))
        );
        assert_eq!(
            parse_json(r#" {"a": null "b": null} "#),
            unable_to_parse_err(),
        );
        assert_eq!(
            parse_json(r#" {"a": null, "a": null} "#),
            Ok(JsonValue::Object(vec![
                (String::from("a"), JsonValue::Null),
                (String::from("a"), JsonValue::Null)
            ]))
        );
    }

    #[test]
    fn test_complex_object_parse() {
        assert_eq!(
            parse_json(
                r#" {
                    "a": [
                        -1.2,
                        "2",
                        false,
                        null
                    ],
                    "b": {

                    },
                    "c": {
                        "nested": {
                            "value": null
                        }
                    }
                } "#
            ),
            Ok(JsonValue::Object(vec![
                (
                    String::from("a"),
                    JsonValue::Array(vec![
                        JsonValue::Number(JsonNumber::F64(-1.2)),
                        JsonValue::String(String::from("2")),
                        JsonValue::Boolean(false),
                        JsonValue::Null,
                    ]),
                ),
                (String::from("b"), JsonValue::Object(vec![])),
                (
                    String::from("c"),
                    JsonValue::Object(vec![(
                        String::from("nested"),
                        JsonValue::Object(vec![(String::from("value"), JsonValue::Null)])
                    )])
                ),
            ]))
        );
    }
}
