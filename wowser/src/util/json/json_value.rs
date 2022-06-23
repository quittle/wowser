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

/// Mapping of JSON string escape sequences to their literal value. Be careful of the ordering of
/// this when processing to avoid "\\" sequences from colliding, generating the wrong string.
///
/// TODO: Handle hex digits \u1234
pub(super) const JSON_STRING_ESCAPE_MAPPING: &[(&str, &str)] = &[
    (r"\\", "\\"),
    (r#"\""#, "\""),
    (r"\/", "/"),
    (r"\b", "\x08"),
    (r"\f", "\x0C"),
    (r"\n", "\n"),
    (r"\r", "\r"),
];

impl JsonValue {
    fn string_escape(string: &str) -> String {
        let mut ret = String::from(string);
        for (escape_sequence, literal) in JSON_STRING_ESCAPE_MAPPING {
            ret = ret.replace(literal, escape_sequence);
        }
        ret
    }

    /// Helper function to stringify JSON values with entries like arrays and objects
    fn pretty_stringify_nested<T, F>(
        does_nesting: bool,
        outer_object_indentation_spaces: &str,
        out: &mut String,
        open: &str,
        close: &str,
        entries: &[T],
        on_entry: F,
    ) where
        F: Fn(&mut String, &T),
    {
        *out += open;
        if does_nesting && (!entries.is_empty()) {
            *out += "\n";
        }
        for (index, entry) in entries.iter().enumerate() {
            on_entry(out, entry);
            if index < entries.len() - 1 {
                *out += ",";
            }
            if does_nesting {
                *out += "\n";
            }
        }
        if !entries.is_empty() {
            *out += outer_object_indentation_spaces;
        }
        *out += close;
    }

    fn pretty_stringify_impl(&self, indentation: usize, cur_level: usize, out: &mut String) {
        let indentation_spaces = " ".repeat(indentation * cur_level);
        let next_indentation_spaces = " ".repeat(indentation * (cur_level + 1));

        match self {
            Self::Object(entries) => {
                JsonValue::pretty_stringify_nested(
                    indentation > 0,
                    &indentation_spaces,
                    out,
                    "{",
                    "}",
                    entries,
                    |out, (key, value)| {
                        *out += &next_indentation_spaces;
                        *out += "\"";
                        *out += &JsonValue::string_escape(key);
                        *out += "\":";
                        if indentation > 0 {
                            *out += " ";
                        }
                        value.pretty_stringify_impl(indentation, cur_level + 1, out);
                    },
                );
            }
            Self::Array(entries) => {
                JsonValue::pretty_stringify_nested(
                    indentation > 0,
                    &indentation_spaces,
                    out,
                    "[",
                    "]",
                    entries,
                    |out, entry| {
                        *out += &next_indentation_spaces;
                        entry.pretty_stringify_impl(indentation, cur_level + 1, out);
                    },
                );
            }
            Self::String(string) => {
                *out += "\"";
                *out += &JsonValue::string_escape(string);
                *out += "\"";
            }
            Self::Number(number) => {
                match number {
                    JsonNumber::F64(f64) => *out += &f64.to_string(),
                    JsonNumber::I64(i64) => *out += &i64.to_string(),
                    JsonNumber::String(string) => *out += string,
                };
            }
            Self::Boolean(bool) => {
                *out += &bool.to_string();
            }
            Self::Null => {
                *out += "null";
            }
        }
    }

    pub fn pretty_stringify(&self, indentation: usize) -> String {
        let mut out_string = String::new();
        self.pretty_stringify_impl(indentation, 0, &mut out_string);
        out_string
    }

    pub fn stringify(&self) -> String {
        self.pretty_stringify(0)
    }
}

impl ToString for JsonValue {
    fn to_string(&self) -> String {
        self.stringify()
    }
}

#[cfg(test)]
mod tests {
    use super::super::JsonNumber;
    use super::JsonValue;

    #[test]
    fn test_to_string_null() {
        assert_eq!(JsonValue::Null.pretty_stringify(0), "null");
        assert_eq!(JsonValue::Null.pretty_stringify(2), "null");
    }

    #[test]
    fn test_to_string_boolean() {
        assert_eq!(JsonValue::Boolean(true).pretty_stringify(0), "true");
        assert_eq!(JsonValue::Boolean(false).pretty_stringify(2), "false");
    }

    #[test]
    fn test_to_string_number() {
        assert_eq!(
            JsonValue::Number(JsonNumber::F64(-12.34)).pretty_stringify(0),
            "-12.34"
        );
        assert_eq!(
            JsonValue::Number(JsonNumber::I64(1234)).pretty_stringify(2),
            "1234"
        );
        assert_eq!(
            JsonValue::Number(JsonNumber::String("1234567890".into())).pretty_stringify(2),
            "1234567890"
        );
    }

    #[test]
    fn test_to_string_string() {
        assert_eq!(
            JsonValue::String("abc 123".into()).pretty_stringify(0),
            r#""abc 123""#
        );
        assert_eq!(
            JsonValue::String(
                r#"abc\
 123\n"#
                    .into()
            )
            .pretty_stringify(2),
            r#""abc\\\n 123\\n""#
        );
    }

    #[test]
    fn test_to_string_array() {
        assert_eq!(JsonValue::Array(vec![]).pretty_stringify(0), "[]");
        assert_eq!(JsonValue::Array(vec![]).pretty_stringify(2), "[]");
        assert_eq!(
            JsonValue::Array(vec![JsonValue::Null]).pretty_stringify(0),
            "[null]"
        );
        assert_eq!(
            JsonValue::Array(vec![JsonValue::Null]).pretty_stringify(2),
            r"[
  null
]"
        );
        assert_eq!(
            JsonValue::Array(vec![JsonValue::Null, JsonValue::Boolean(true)]).pretty_stringify(0),
            "[null,true]"
        );
        assert_eq!(
            JsonValue::Array(vec![JsonValue::Null, JsonValue::Boolean(true)]).pretty_stringify(2),
            r"[
  null,
  true
]"
        );
        assert_eq!(
            JsonValue::Array(vec![
                JsonValue::Array(vec![JsonValue::Null, JsonValue::Boolean(true)]),
                JsonValue::Array(vec![])
            ])
            .pretty_stringify(2),
            r"[
  [
    null,
    true
  ],
  []
]"
        );
    }

    #[test]
    fn test_to_string_object() {
        let empty = JsonValue::Object(vec![]);
        assert_eq!(empty.pretty_stringify(0), "{}");
        assert_eq!(empty.pretty_stringify(2), "{}");
        let single_entry = JsonValue::Object(vec![("null".into(), JsonValue::Null)]);
        assert_eq!(single_entry.pretty_stringify(0), r#"{"null":null}"#);
        assert_eq!(
            single_entry.pretty_stringify(2),
            r#"{
  "null": null
}"#
        );
        let multi_entry = JsonValue::Object(vec![
            ("".into(), JsonValue::Null),
            ("other".into(), JsonValue::Boolean(true)),
        ]);
        assert_eq!(multi_entry.pretty_stringify(0), r#"{"":null,"other":true}"#);
        assert_eq!(
            multi_entry.pretty_stringify(2),
            r#"{
  "": null,
  "other": true
}"#
        );
        let nested = JsonValue::Object(vec![
            (
                "nested".into(),
                JsonValue::Object(vec![
                    ("a".into(), JsonValue::Null),
                    ("b".into(), JsonValue::Boolean(true)),
                ]),
            ),
            ("other".into(), JsonValue::Object(vec![])),
        ]);
        assert_eq!(
            nested.pretty_stringify(0),
            r#"{"nested":{"a":null,"b":true},"other":{}}"#
        );
        assert_eq!(
            nested.pretty_stringify(2),
            r#"{
  "nested": {
    "a": null,
    "b": true
  },
  "other": {}
}"#
        );
    }
}
