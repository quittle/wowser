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
    use crate::js::JsValue;

    use super::{parse_js, JsStatementResult};

    fn run_test(script: &str, expected_results: Vec<JsStatementResult>) {
        let mut js_document = parse_js(script).unwrap();
        js_document.run();
        assert_eq!(expected_results, js_document.expression_results);
    }

    #[test]
    fn test_js() {
        run_test("1", vec![JsStatementResult::Value(JsValue::Number(1.0))]);
        // run_test(
        //     "123;1+2+3;;12",
        //     vec![
        //         JsStatementResult::Value(JsValue::Number(123.0)),
        //         JsStatementResult::Value(JsValue::Number(6.0)),
        //         JsStatementResult::Void,
        //         JsStatementResult::Value(JsValue::Number(12.0)),
        //     ],
        // );
    }
}
