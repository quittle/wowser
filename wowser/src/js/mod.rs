mod js_interpreter;
mod js_rule;
mod js_token;
mod runtime;

pub use js_interpreter::*;
pub use js_rule::*;
pub use js_token::*;
pub use runtime::*;

use crate::parse::{Interpreter, Lexer, Parser};

pub fn parse_js(document: &str) -> Result<JsDocument, String> {
    let lexer = Lexer::new(JsToken::Document);
    let tokens = lexer.parse(document).ok_or("Failed to lex JS")?;
    let ast = Parser {}.parse(&tokens, &JsRule::Document)?;
    let document = JsInterpreter {}
        .interpret(&ast)
        .ok_or("Failed to interpret JS")?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::js::JsValue;

    use super::{parse_js, JsExpression, JsFunction, JsStatement, JsStatementResult};

    fn run_js(script: &str) -> Vec<JsStatementResult> {
        let mut js_document = parse_js(script).unwrap();
        js_document.run();
        js_document.expression_results
    }

    fn run_test(script: &str, expected_results: Vec<JsStatementResult>) {
        let results = run_js(script);
        assert_eq!(results, expected_results);
    }

    fn result_as_number(result: &JsStatementResult) -> f64 {
        match result {
            JsStatementResult::Value(v) => match v.as_ref() {
                JsValue::Number(n) => *n,
                v => panic!("Invalid value type: {:?}", v),
            },
            _ => panic!("Required JS value of type number but received {:?}", result),
        }
    }

    #[test]
    fn test_js() {
        run_test("1", vec![JsStatementResult::number(1.0)]);
        run_test(
            "123;;12",
            vec![
                JsStatementResult::number(123.0),
                JsStatementResult::Void,
                JsStatementResult::number(12.0),
            ],
        );
    }

    #[test]
    fn test_numbers() {
        run_test("1", vec![JsStatementResult::number(1.0)]);
        run_test("-1", vec![JsStatementResult::number(-1.0)]);
        run_test("-1_0", vec![JsStatementResult::number(-10.0)]);
        run_test("-1_0.0_1", vec![JsStatementResult::number(-10.01)]);
    }

    #[test]
    fn test_sum() {
        run_test("1+2", vec![JsStatementResult::number(3.0)]);
        run_test("1+2+3", vec![JsStatementResult::number(6.0)]);
    }

    #[test]
    fn test_plus_plus() {
        run_test("1 + +2", vec![JsStatementResult::number(3.0)]);
        assert!(run_js("+'a'")[0].is_nan());
        run_test("+'12'", vec![JsStatementResult::number(12.0)]);
        run_test("var a = +'1'", vec![JsStatementResult::number(1.0)]);
    }

    #[test]
    fn test_multiply() {
        run_test("1 * 2", vec![JsStatementResult::number(2.0)]);
        run_test("1 * 2 * 3 + 4 * 5", vec![JsStatementResult::number(26.0)]);
    }

    #[test]
    fn test_var() {
        run_test("var a", vec![JsStatementResult::undefined()]);
        run_test(
            "1; var abc123 ;",
            vec![
                JsStatementResult::number(1.0),
                JsStatementResult::undefined(),
            ],
        );
        run_test(
            "a = 1; a",
            vec![
                JsStatementResult::number(1.0),
                JsStatementResult::number(1.0),
            ],
        );
        run_test(
            "var a = 1; a",
            vec![
                JsStatementResult::number(1.0),
                JsStatementResult::number(1.0),
            ],
        );
    }

    #[test]
    fn test_var_assigment() {
        run_test(
            "var a = 1; a = 2; a",
            vec![
                JsStatementResult::number(1.0),
                JsStatementResult::number(2.0),
                JsStatementResult::number(2.0),
            ],
        );
        run_test(
            "var a; var b = 2; a = 1; b = a + b; b",
            vec![
                JsStatementResult::undefined(),
                JsStatementResult::number(2.0),
                JsStatementResult::number(1.0),
                JsStatementResult::number(3.0),
                JsStatementResult::number(3.0),
            ],
        );
        let results = run_js("var a; 2 * a + 1");
        assert_eq!(results[0], JsStatementResult::undefined());
        assert!(result_as_number(&results[1]).is_nan());
    }

    #[test]
    pub fn test_string() {
        run_test(
            r#"'abc'; "123"; ''; """#,
            vec![
                JsStatementResult::string("abc"),
                JsStatementResult::string("123"),
                JsStatementResult::string(""),
                JsStatementResult::string(""),
            ],
        );
    }

    #[test]
    pub fn test_string_addition() {
        run_test("'abc' + 'def'", vec![JsStatementResult::string("abcdef")]);
        run_test("'abc' + 123", vec![JsStatementResult::string("abc123")]);
        run_test("123 + 'abc'", vec![JsStatementResult::string("123abc")]);
        run_test("1.23 + ''", vec![JsStatementResult::string("1.23")]);
        run_test(
            "var a; 'oops: ' + a",
            vec![
                JsStatementResult::undefined(),
                JsStatementResult::string("oops: undefined"),
            ],
        );
        run_test(
            "var a; a + ' <- oops'",
            vec![
                JsStatementResult::undefined(),
                JsStatementResult::string("undefined <- oops"),
            ],
        );
        run_test("'1' + '2'", vec![JsStatementResult::string("12")]);
    }

    #[test]
    pub fn test_string_multiplication() {
        assert!(run_js("'abc' * 'def'")[0].is_nan());
        assert!(run_js("'abc' * 123")[0].is_nan());
        assert!(run_js("123 * 'abc'")[0].is_nan());
        run_test("1.23 * ''", vec![JsStatementResult::number(0.0)]);
        assert!(run_js("var a; 'abc' * a")[1].is_nan());
        assert!(run_js("var a; '123' * a")[1].is_nan());
        run_test("'2 ' * ' 3 '", vec![JsStatementResult::number(6.0)]);
        run_test("'2 ' * 3", vec![JsStatementResult::number(6.0)]);
    }

    #[test]
    pub fn test_global_function() {
        run_test(
            "atob(btoa('a' + 'b' + 'c') + 'de')",
            vec![JsStatementResult::string("abcu")],
        );
    }

    #[test]
    pub fn test_function_declaration() {
        run_test(
            "function foo(arg1, arg2) { arg1 + arg2; }",
            vec![JsStatementResult::Value(Rc::new(JsValue::Function(
                JsFunction::UserDefined(
                    "foo".to_string(),
                    vec!["arg1".to_string(), "arg2".to_string()],
                    vec![JsStatement::Expression(JsExpression::Add(
                        Box::new(JsExpression::Reference("arg1".to_string())),
                        Box::new(JsExpression::Reference("arg2".to_string())),
                    ))],
                ),
            )))],
        )
    }
}
