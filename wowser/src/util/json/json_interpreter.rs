use std::ops::AddAssign;

use super::{
    json_number::JsonNumber,
    json_rule::JsonRule,
    json_token::JsonToken,
    json_value::{JsonValue, JSON_STRING_ESCAPE_MAPPING},
};
use crate::parse::*;

pub(super) struct JsonInterpreter {}

/// Parses a JSON string into a Rust string, evaluating escape sequences.
fn expect_string_literal(value: &str) -> String {
    let trimmed = value[1..value.len() - 1].to_string(); // Trim quotes from beginning and end
    let mut output = String::with_capacity(trimmed.len());

    // This complication is required instead of the more obvious sequence of replace statements to
    // avoid sequences like "\\n" being interpreted either as
    // - <newline> (escaping backslash, then newline)
    // - \<newline> (escaping newline, then backslash)
    // when a given character should only be evaluated once.
    let mut i = 0;
    while i < trimmed.len() {
        for (escape_sequence, literal) in JSON_STRING_ESCAPE_MAPPING {
            if trimmed[i..].starts_with(escape_sequence) {
                output.add_assign(literal);
                i += escape_sequence.len();
                continue;
            }
        }
        output.add_assign(&trimmed[i..=i]);
        i += 1;
    }

    output
}

fn expect_number_literal(value: &str) -> JsonNumber {
    if let Ok(parsed_value) = value.parse::<i64>() {
        JsonNumber::I64(parsed_value)
    } else if let Ok(parsed_value) = value.parse::<f64>() {
        if parsed_value.is_finite() {
            JsonNumber::F64(parsed_value)
        } else {
            JsonNumber::String(value.to_string())
        }
    } else {
        JsonNumber::String(value.to_string())
    }
}

fn expect_bool_literal(value: &str) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => panic!("Invalid boolean literal: {value}"),
    }
}

fn on_literal(ast: &ASTNode<JsonRule>) -> JsonValue {
    extract_interpreter_n_children(ast, JsonRule::Literal, 0);

    let token = ast.token.expect("Missing required token");
    match token.token {
        JsonToken::String => JsonValue::String(expect_string_literal(token.literal)),
        JsonToken::Number => JsonValue::Number(expect_number_literal(token.literal)),
        JsonToken::Boolean => JsonValue::Boolean(expect_bool_literal(token.literal)),
        JsonToken::Null => JsonValue::Null,
        token => panic!("Unexpected token received {token}"),
    }
}

fn on_array(ast: &ASTNode<JsonRule>) -> JsonValue {
    let children = extract_interpreter_n_children(ast, JsonRule::Array, 3);

    JsonValue::Array(on_array_entries(&children[1]))
}

fn on_array_entries(ast: &ASTNode<JsonRule>) -> Vec<JsonValue> {
    let children = extract_interpreter_children(ast, JsonRule::ArrayEntries);

    if children.is_empty() {
        return vec![];
    }

    let mut ret = vec![on_value(&children[0])];

    if children.len() == 3 {
        ret.extend(on_array_entries(&children[2]));
    }

    ret
}

fn on_value(ast: &ASTNode<JsonRule>) -> JsonValue {
    let children = extract_interpreter_n_children(ast, JsonRule::Value, 1);

    let child = &children[0];
    match child.rule {
        JsonRule::Literal => on_literal(child),
        JsonRule::Array => on_array(child),
        JsonRule::Object => on_object(child),
        rule => panic!("Unexpected child rule for value: {rule}"),
    }
}

fn on_object(ast: &ASTNode<JsonRule>) -> JsonValue {
    let children = extract_interpreter_n_children(ast, JsonRule::Object, 3);

    JsonValue::Object(on_object_entries(&children[1]))
}

fn on_object_entries(ast: &ASTNode<JsonRule>) -> Vec<(String, JsonValue)> {
    let children = extract_interpreter_children(ast, JsonRule::ObjectEntries);

    if children.is_empty() {
        return vec![];
    }

    let mut ret = vec![on_object_entry(&children[0])];

    if children.len() == 3 {
        ret.extend(on_object_entries(&children[2]));
    }

    ret
}

fn on_object_entry(ast: &ASTNode<JsonRule>) -> (String, JsonValue) {
    let children = extract_interpreter_n_children(ast, JsonRule::ObjectEntry, 3);

    let key = on_string_token(&children[0]);
    let value = on_value(&children[2]);

    (key, value)
}

fn on_string_token(ast: &ASTNode<JsonRule>) -> String {
    let token = extract_interpreter_token(ast, JsonRule::StringToken);
    expect_string_literal(&token)
}

impl Interpreter<'_, JsonRule> for JsonInterpreter {
    type Result = JsonValue;

    fn on_node(&self, ast: &ASTNode<JsonRule>) -> Option<Self::Result> {
        let children = extract_interpreter_n_children(ast, JsonRule::Document, 2);

        let first_child = &children[0];
        Some(match first_child.rule {
            JsonRule::Literal => on_literal(first_child),
            JsonRule::Array => on_array(first_child),
            JsonRule::Object => on_object(first_child),
            rule => unreachable!("Invalid child rule type {}", rule),
        })
    }
}
