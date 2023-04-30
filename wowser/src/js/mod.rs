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
    use std::collections::HashMap;

    use crate::garbage_collector::GcNodeGraph;

    use super::{
        parse_js, JsExpression, JsFunction, JsStatement, JsStatementResult, JsValue, JsValueGraph,
    };

    fn get_node_graph() -> JsValueGraph {
        let (node_graph, _root) = GcNodeGraph::new(JsValue::Undefined);
        node_graph
    }

    /// This returns the value graph because when it goes out of scope, the results get cleared from memory
    #[track_caller]
    fn run_js(script: &str) -> (JsValueGraph, Vec<JsStatementResult>) {
        let mut js_document = parse_js(script).unwrap();
        js_document.run();
        (
            js_document.global_closure_context.nodes_graph,
            js_document.global_closure_context.expression_results,
        )
    }

    #[track_caller]
    fn run_test(script: &str, expected_results: Vec<JsStatementResult>) {
        let (_node_graph, results) = run_js(script);
        assert_eq!(results, expected_results);
    }

    #[track_caller]
    fn assert_last_value_equals(script: &str, expected_result: JsStatementResult) {
        let (_node_graph, results) = run_js(script);
        assert_eq!(results.last().unwrap(), &expected_result);
    }

    fn result_as_number(result: &JsStatementResult) -> f64 {
        match result {
            JsStatementResult::Value(v) => match v.get_ref() {
                JsValue::Number(n) => *n,
                v => panic!("Invalid value type: {:?}", v),
            },
            _ => panic!("Required JS value of type number but received {:?}", result),
        }
    }

    #[test]
    fn test_comments() {
        let node_graph = get_node_graph();
        run_test("//", vec![]);
        run_test("// abc", vec![]);
        run_test("/**/", vec![]);
        run_test("/* abc */", vec![]);
        run_test(
            "/* abc
                               */",
            vec![],
        );
        run_test(
            "1; /* abc
                                */2",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::number(&node_graph, 2),
            ],
        );
        run_test(
            "var // abc
             a = /* def */ 1; // ghi",
            vec![JsStatementResult::number(&node_graph, 1)],
        );
        run_test(
            "var /*
                        */ a = /* def */ 1; // ghi",
            vec![JsStatementResult::number(&node_graph, 1)],
        );
        run_test(
            "var /* space -> 
                        */ a = /* def */ 1; // ghi",
            vec![JsStatementResult::number(&node_graph, 1)],
        );
    }

    #[test]
    fn test_js() {
        let node_graph = get_node_graph();
        run_test("1", vec![JsStatementResult::number(&node_graph, 1.0)]);
        run_test(
            "123;;12",
            vec![
                JsStatementResult::number(&node_graph, 123.0),
                JsStatementResult::Void,
                JsStatementResult::number(&node_graph, 12.0),
            ],
        );
    }

    #[test]
    fn test_numbers() {
        let node_graph = get_node_graph();
        run_test("1", vec![JsStatementResult::number(&node_graph, 1.0)]);
        run_test("-1", vec![JsStatementResult::number(&node_graph, -1.0)]);
        run_test("-1_0", vec![JsStatementResult::number(&node_graph, -10.0)]);
        run_test(
            "-1_0.0_1",
            vec![JsStatementResult::number(&node_graph, -10.01)],
        );
    }

    #[test]
    fn test_sum() {
        let node_graph = get_node_graph();
        run_test("1+2", vec![JsStatementResult::number(&node_graph, 3.0)]);
        run_test("1+2+3", vec![JsStatementResult::number(&node_graph, 6.0)]);
    }

    #[test]
    fn test_plus_plus() {
        let node_graph = get_node_graph();
        run_test("1 + +2", vec![JsStatementResult::number(&node_graph, 3.0)]);
        assert!(run_js("+'a'").1[0].is_nan());
        run_test("+'12'", vec![JsStatementResult::number(&node_graph, 12.0)]);
        run_test(
            "var a = +'1'",
            vec![JsStatementResult::number(&node_graph, 1.0)],
        );
    }

    #[test]
    fn test_multiply() {
        let node_graph = get_node_graph();
        run_test("1 * 2", vec![JsStatementResult::number(&node_graph, 2.0)]);
        run_test(
            "1 * 2 * 3 + 4 * 5",
            vec![JsStatementResult::number(&node_graph, 26.0)],
        );
    }

    #[test]
    fn test_var() {
        let node_graph = get_node_graph();
        run_test("var a", vec![JsStatementResult::undefined(&node_graph)]);
        run_test(
            "1; var abc123 ;",
            vec![
                JsStatementResult::number(&node_graph, 1.0),
                JsStatementResult::undefined(&node_graph),
            ],
        );
        run_test(
            "a = 1; a",
            vec![
                JsStatementResult::number(&node_graph, 1.0),
                JsStatementResult::number(&node_graph, 1.0),
            ],
        );
        run_test(
            "var a = 1; a",
            vec![
                JsStatementResult::number(&node_graph, 1.0),
                JsStatementResult::number(&node_graph, 1.0),
            ],
        );
    }

    #[test]
    fn test_var_assigment() {
        let node_graph = get_node_graph();
        run_test(
            "var a = 1; a = 2; a",
            vec![
                JsStatementResult::number(&node_graph, 1.0),
                JsStatementResult::number(&node_graph, 2.0),
                JsStatementResult::number(&node_graph, 2.0),
            ],
        );
        run_test(
            "var a; var b = 2; a = 1; b = a + b; b",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::number(&node_graph, 2.0),
                JsStatementResult::number(&node_graph, 1.0),
                JsStatementResult::number(&node_graph, 3.0),
                JsStatementResult::number(&node_graph, 3.0),
            ],
        );
        let results = run_js("var a; 2 * a + 1");
        assert_eq!(results.1[0], JsStatementResult::undefined(&node_graph,));
        assert!(result_as_number(&results.1[1]).is_nan());
    }

    #[test]
    pub fn test_string() {
        let node_graph = get_node_graph();
        run_test(
            r#"'abc'; "123"; ''; """#,
            vec![
                JsStatementResult::string(&node_graph, "abc"),
                JsStatementResult::string(&node_graph, "123"),
                JsStatementResult::string(&node_graph, ""),
                JsStatementResult::string(&node_graph, ""),
            ],
        );
    }

    #[test]
    pub fn test_string_addition() {
        let node_graph = get_node_graph();
        run_test(
            "'abc' + 'def'",
            vec![JsStatementResult::string(&node_graph, "abcdef")],
        );
        run_test(
            "'abc' + 123",
            vec![JsStatementResult::string(&node_graph, "abc123")],
        );
        run_test(
            "123 + 'abc'",
            vec![JsStatementResult::string(&node_graph, "123abc")],
        );
        run_test(
            "1.23 + ''",
            vec![JsStatementResult::string(&node_graph, "1.23")],
        );
        run_test(
            "var a; 'oops: ' + a",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::string(&node_graph, "oops: undefined"),
            ],
        );
        run_test(
            "var a; a + ' <- oops'",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::string(&node_graph, "undefined <- oops"),
            ],
        );
        run_test(
            "'1' + '2'",
            vec![JsStatementResult::string(&node_graph, "12")],
        );
    }

    #[test]
    pub fn test_string_multiplication() {
        let node_graph = get_node_graph();
        assert!(run_js("'abc' * 'def'").1[0].is_nan());
        assert!(run_js("'abc' * 123").1[0].is_nan());
        assert!(run_js("123 * 'abc'").1[0].is_nan());
        run_test(
            "1.23 * ''",
            vec![JsStatementResult::number(&node_graph, 0.0)],
        );
        assert!(run_js("var a; 'abc' * a").1[1].is_nan());
        assert!(run_js("var a; '123' * a").1[1].is_nan());
        run_test(
            "'2 ' * ' 3 '",
            vec![JsStatementResult::number(&node_graph, 6.0)],
        );
        run_test(
            "'2 ' * 3",
            vec![JsStatementResult::number(&node_graph, 6.0)],
        );
    }

    #[test]
    pub fn test_global_function() {
        let node_graph = get_node_graph();
        run_test(
            "atob(btoa('a' + 'b' + 'c') + 'de')",
            vec![JsStatementResult::string(&node_graph, "abcu")],
        );
    }

    #[test]
    pub fn test_function_declaration() {
        let node_graph = get_node_graph();
        run_test(
            "function foo(){}",
            vec![JsStatementResult::Value(GcNodeGraph::create_node(
                &node_graph,
                JsValue::Function(JsFunction::UserDefined(
                    "function foo(){}".to_string(),
                    "foo".to_string(),
                    vec![],
                    vec![],
                )),
            ))],
        );
        run_test(
            "function foo(arg1, arg2) { arg1 + arg2; }",
            vec![JsStatementResult::Value(GcNodeGraph::create_node(
                &node_graph,
                JsValue::Function(JsFunction::UserDefined(
                    "function foo(arg1, arg2) { arg1 + arg2; }".to_string(),
                    "foo".to_string(),
                    vec!["arg1".to_string(), "arg2".to_string()],
                    vec![JsStatement::Expression(JsExpression::Add(
                        Box::new(JsExpression::Reference("arg1".to_string())),
                        Box::new(JsExpression::Reference("arg2".to_string())),
                    ))],
                )),
            ))],
        );
        run_test(
            "function foo(arg1, arg2) { arg1 + arg2; };function bar(arg1, arg2) { arg1 + arg2; }",
            vec![
                JsStatementResult::Value(GcNodeGraph::create_node(
                    &node_graph,
                    JsValue::Function(JsFunction::UserDefined(
                        "function foo(arg1, arg2) { arg1 + arg2; }".to_string(),
                        "foo".to_string(),
                        vec!["arg1".to_string(), "arg2".to_string()],
                        vec![JsStatement::Expression(JsExpression::Add(
                            Box::new(JsExpression::Reference("arg1".to_string())),
                            Box::new(JsExpression::Reference("arg2".to_string())),
                        ))],
                    )),
                )),
                JsStatementResult::Void,
                JsStatementResult::Value(GcNodeGraph::create_node(
                    &node_graph,
                    JsValue::Function(JsFunction::UserDefined(
                        "function bar(arg1, arg2) { arg1 + arg2; }".to_string(),
                        "bar".to_string(),
                        vec!["arg1".to_string(), "arg2".to_string()],
                        vec![JsStatement::Expression(JsExpression::Add(
                            Box::new(JsExpression::Reference("arg1".to_string())),
                            Box::new(JsExpression::Reference("arg2".to_string())),
                        ))],
                    )),
                )),
            ],
        );
    }

    #[test]
    pub fn test_user_function_invocations() {
        let node_graph = get_node_graph();
        run_test(
            "function foo(arg1, arg2) { arg1 + arg2; } foo(1, 'abc')",
            vec![
                JsStatementResult::Value(GcNodeGraph::create_node(
                    &node_graph,
                    JsValue::Function(JsFunction::UserDefined(
                        "function foo(arg1, arg2) { arg1 + arg2; }".to_string(),
                        "foo".to_string(),
                        vec!["arg1".to_string(), "arg2".to_string()],
                        vec![JsStatement::Expression(JsExpression::Add(
                            Box::new(JsExpression::Reference("arg1".to_string())),
                            Box::new(JsExpression::Reference("arg2".to_string())),
                        ))],
                    )),
                )),
                JsStatementResult::string(&node_graph, "1abc"),
                JsStatementResult::undefined(&node_graph),
            ],
        );

        run_test(
            "function foo(arg1, arg2) { arg1 + arg2; return arg1; arg2; return arg2; } foo(1, 'abc')",
            vec![
                JsStatementResult::Value(GcNodeGraph::create_node(
                    &node_graph,
                    JsValue::Function(JsFunction::UserDefined(
                    "function foo(arg1, arg2) { arg1 + arg2; return arg1; arg2; return arg2; }".to_string(),
                    "foo".to_string(),
                    vec!["arg1".to_string(), "arg2".to_string()],
                    vec![
                        JsStatement::Expression(JsExpression::Add(
                            Box::new(JsExpression::Reference("arg1".to_string())),
                            Box::new(JsExpression::Reference("arg2".to_string())),
                        )),
                        JsStatement::Return(JsExpression::Reference("arg1".to_string())),
                        JsStatement::Expression(JsExpression::Reference("arg2".to_string())),
                        JsStatement::Return(JsExpression::Reference("arg2".to_string())),
                    ],
                )))),
                JsStatementResult::string(&node_graph, "1abc"),
                JsStatementResult::number(&node_graph, 1),
            ],
        );
    }

    #[test]
    fn test_undefined() {
        let node_graph = get_node_graph();
        run_test("undefined", vec![JsStatementResult::undefined(&node_graph)]);
        run_test(
            "undefined; undefined",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::undefined(&node_graph),
            ],
        );
        run_test(
            "var a; a = undefined; a",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::undefined(&node_graph),
            ],
        );
        assert!(run_js("undefined+undefined*undefined+undefined",).1[0].is_nan());
        run_test(
            "var undefined_var = 1; undefined_var",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::number(&node_graph, 1),
            ],
        );
    }

    #[test]
    fn test_bool() {
        let node_graph = get_node_graph();
        run_test("true", vec![JsStatementResult::bool(&node_graph, true)]);
        run_test("false", vec![JsStatementResult::bool(&node_graph, false)]);
        run_test(
            "false + true;",
            vec![JsStatementResult::number(&node_graph, 1)],
        );
        run_test(
            "true + false",
            vec![JsStatementResult::number(&node_graph, 1)],
        );
        run_test(
            "1 * false",
            vec![JsStatementResult::number(&node_graph, 0.0)],
        );
        assert!(run_js("undefined * false").1[0].is_nan());
        run_test(
            "true + '2'",
            vec![JsStatementResult::string(&node_graph, "true2")],
        );
    }

    #[test]
    fn test_null() {
        let node_graph = get_node_graph();
        run_test("null", vec![JsStatementResult::null(&node_graph)]);
        run_test("null;", vec![JsStatementResult::null(&node_graph)]);
        run_test("1 + null;", vec![JsStatementResult::number(&node_graph, 1)]);
        run_test("3 * null", vec![JsStatementResult::number(&node_graph, 0)]);
        assert!(run_js("null + undefined").1[0].is_nan());
        run_test(
            "null + ''",
            vec![JsStatementResult::string(&node_graph, "null")],
        );
        run_test("null * ''", vec![JsStatementResult::number(&node_graph, 0)]);
    }

    #[test]
    fn test_nan() {
        let _node_graph = get_node_graph();
        assert!(run_js("NaN").1[0].is_nan());
        assert!(run_js("NaN * NaN + NaN").1[0].is_nan());
    }

    #[test]
    fn test_if_statements() {
        let node_graph = get_node_graph();
        run_test(
            "if(true){1;}",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(true)1",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(true)var i;2",
            vec![
                JsStatementResult::undefined(&node_graph),
                JsStatementResult::Void,
                JsStatementResult::number(&node_graph, 2),
            ],
        );
        run_test(
            "1; if (null * undefined) { 2; } 3",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::Void,
                JsStatementResult::number(&node_graph, 3),
            ],
        );
    }

    #[test]
    fn test_else_statements() {
        let node_graph = get_node_graph();
        run_test(
            "if(true){1;} else {2;}",
            vec![
                JsStatementResult::number(&node_graph, 1),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(false){1;} else {2;}",
            vec![
                JsStatementResult::number(&node_graph, 2),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(false)1; else 2;",
            vec![
                JsStatementResult::number(&node_graph, 2),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(false)var i;else 2",
            vec![
                JsStatementResult::number(&node_graph, 2),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if(false){2;}else 3",
            vec![
                JsStatementResult::number(&node_graph, 3),
                JsStatementResult::Void,
            ],
        );
        run_test(
            "if (false) 1; else { 2; } 3",
            vec![
                JsStatementResult::number(&node_graph, 2),
                JsStatementResult::Void,
                JsStatementResult::number(&node_graph, 3),
            ],
        );
    }

    #[test]
    fn test_triple_equality() {
        let node_graph = get_node_graph();
        run_test("1 === 1", vec![JsStatementResult::bool(&node_graph, true)]);
        // NaN inequality
        run_test(
            "1*undefined === 1*undefined",
            vec![JsStatementResult::bool(&node_graph, false)],
        );
        run_test("1 === 01", vec![JsStatementResult::bool(&node_graph, true)]);
        run_test(
            "'abc' === \"abc\"",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        assert_eq!(
            run_js("function abc(){} var cba = abc; abc === cba").1[2],
            JsStatementResult::bool(&node_graph, true),
        );
        run_test(
            "true === true",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "true !== false",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "null === null",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "undefined === undefined",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "undefined === null",
            vec![JsStatementResult::bool(&node_graph, false)],
        );
        assert_last_value_equals("{} == {}", JsStatementResult::bool(&node_graph, false));
        assert_last_value_equals(
            "var a = {}; var b = a; a == b",
            JsStatementResult::bool(&node_graph, true),
        );

        run_test("1 !== 1", vec![JsStatementResult::bool(&node_graph, false)]);
    }

    #[test]
    fn test_double_equality() {
        let node_graph = get_node_graph();
        run_test("1 == 1", vec![JsStatementResult::bool(&node_graph, true)]);
        // NaN inequality
        run_test(
            "1*undefined == 1*undefined",
            vec![JsStatementResult::bool(&node_graph, false)],
        );
        run_test("1 == 01", vec![JsStatementResult::bool(&node_graph, true)]);
        run_test(
            "'abc' == \"abc\"",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        assert_eq!(
            run_js("function abc(){} var cba = abc; abc == cba").1[2],
            JsStatementResult::bool(&node_graph, true),
        );
        run_test(
            "true == true",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "true != false",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "null == null",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "undefined == undefined",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "undefined == null",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        run_test(
            "{} == {}",
            vec![JsStatementResult::bool(&node_graph, false)],
        );
        assert_last_value_equals(
            "var a = {}; var b = a; a == b",
            JsStatementResult::bool(&node_graph, true),
        );
        assert_last_value_equals(
            "{} + ''",
            JsStatementResult::string(&node_graph, "[object Object]"),
        );

        run_test("1 != 1", vec![JsStatementResult::bool(&node_graph, false)]);

        run_test(
            "1 == ' 01  '",
            vec![JsStatementResult::bool(&node_graph, true)],
        );
        assert_eq!(
            run_js("  function  abc ( ) { } abc == 'function  abc ( ) { }'").1[1],
            JsStatementResult::bool(&node_graph, true),
        );
    }

    #[test]
    fn test_recursion() {
        let node_graph = get_node_graph();
        assert_last_value_equals(
            "function factorial(num) {if (num == 1) { return 1; } return num * factorial(num + -1);} factorial(5)",
            JsStatementResult::number(&node_graph, 120),
        );
        // Infinite recursion leads to undefined right now instead of a stack overflow exception
        assert_last_value_equals(
            "function recurse(num) {recurse(num + 1);} recurse(1)",
            JsStatementResult::ThrowValue(JsValue::stack_overflow_error_rc(&node_graph)),
        );
    }

    #[test]
    fn test_object_literal() {
        let node_graph = get_node_graph();
        run_test("{}", vec![JsStatementResult::object(&node_graph, vec![])]);
        run_test(
            " { 'a' : 1 } ",
            vec![JsStatementResult::object(
                &node_graph,
                vec![("a", JsValue::number_rc(&node_graph, 1))],
            )],
        );
        run_test(
            "{'a': 1, 'b': 2}",
            vec![JsStatementResult::object(
                &node_graph,
                vec![
                    ("a", JsValue::number_rc(&node_graph, 1)),
                    ("b", JsValue::number_rc(&node_graph, 2)),
                ],
            )],
        );
        run_test(
            "{'a': 1, 'b': 2,}",
            vec![JsStatementResult::object(
                &node_graph,
                vec![
                    ("a", JsValue::number_rc(&node_graph, 1)),
                    ("b", JsValue::number_rc(&node_graph, 2)),
                ],
            )],
        );
        run_test(
            "{'a': 1, 'a': 2}",
            vec![JsStatementResult::object(
                &node_graph,
                vec![("a", JsValue::number_rc(&node_graph, 2))],
            )],
        );
        run_test(
            "{'a': 1 + 2, 'b': {}}",
            vec![JsStatementResult::object(
                &node_graph,
                vec![
                    ("a", JsValue::number_rc(&node_graph, 3)),
                    ("b", JsValue::object_rc(&node_graph, HashMap::new())),
                ],
            )],
        );
    }

    #[test]
    fn test_object_operations() {
        let node_graph = get_node_graph();
        assert!(run_js("5 * {}").1.last().unwrap().is_nan());
        run_test(
            "5 + {}",
            vec![JsStatementResult::string(&node_graph, "5[object Object]")],
        );
    }

    #[test]
    fn test_object_access() {
        let node_graph = get_node_graph();
        assert_last_value_equals(
            "var a=true; a.foo",
            JsStatementResult::undefined(&node_graph),
        );
        assert_last_value_equals(
            "var a=123; a.foo",
            JsStatementResult::undefined(&node_graph),
        );
        assert_last_value_equals(
            "var a='abc'; a.foo",
            JsStatementResult::undefined(&node_graph),
        );
        assert_last_value_equals("var a={}; a.foo", JsStatementResult::undefined(&node_graph));
        assert_last_value_equals(
            "var a={'not foo': 'bar'}; a.foo",
            JsStatementResult::undefined(&node_graph),
        );
        assert_last_value_equals(
            "var a={'foo': 'bar'}; a.foo",
            JsStatementResult::string(&node_graph, "bar"),
        );
        assert_last_value_equals(
            "var a={'foo': {'bar': 123}}; a.foo.bar",
            JsStatementResult::number(&node_graph, 123),
        );
    }

    #[test]
    fn test_prototype_to_string() {
        let node_graph = get_node_graph();
        assert_last_value_equals(
            "var a = true; a.toString()",
            JsStatementResult::string(&node_graph, "true"),
        );
        assert_last_value_equals(
            "var a = 5; a.toString()",
            JsStatementResult::string(&node_graph, "5"),
        );
        assert_last_value_equals(
            "var a = {}; a.toString()",
            JsStatementResult::string(&node_graph, "[object Object]"),
        );
    }

    #[test]
    fn test_condition() {
        let node_graph = get_node_graph();
        assert_last_value_equals("1 ? 2 : 3", JsStatementResult::number(&node_graph, 2));
        assert_last_value_equals("false ? 2 : 3", JsStatementResult::number(&node_graph, 3));
        assert_last_value_equals(
            "1 ? 0 ? 3 : 4 : 5",
            JsStatementResult::number(&node_graph, 4),
        );
        assert_last_value_equals(
            "1 + 1 ? 2 + 2 : 3 + 3",
            JsStatementResult::number(&node_graph, 4),
        );
    }

    #[test]
    fn test_throw() {
        let node_graph = get_node_graph();
        assert_last_value_equals(
            "throw 1;",
            JsStatementResult::ThrowValue(JsValue::number_rc(&node_graph, 1)),
        );
        assert_last_value_equals(
            "function throws(val) { throw val; } throws('abc'); ",
            JsStatementResult::ThrowValue(JsValue::str_rc(&node_graph, "abc")),
        );
        assert_last_value_equals(
            "function throws(val) { if (val) { throw val; } } throws(''); ",
            JsStatementResult::Value(JsValue::undefined_rc(&node_graph)),
        );
    }

    #[ignore = "Currently only has syntax parsing support"]
    #[test]
    fn test_this() {
        let node_graph = get_node_graph();
        assert_last_value_equals(
            "this;",
            JsStatementResult::Value(JsValue::object_rc(&node_graph, Default::default())),
        );
    }

    /// Note that this test should eventually fail
    #[test]
    fn test_garbage_collection() {
        let (node_graph, results) = run_js("123");
        let expected_result = JsStatementResult::number(&node_graph, 123);
        let actual_result = results.last().unwrap();
        assert_eq!(actual_result, &expected_result);

        // This shrinks as all the globals get discarded and only the root node remains.
        assert_eq!(node_graph.borrow().size(), 14);
        GcNodeGraph::gc(&node_graph);
        assert_eq!(node_graph.borrow().size(), 1);

        if let JsStatementResult::Value(node) = actual_result {
            assert!(!node.exists());
        } else {
            panic!("Invalid type of actual result");
        }
    }
}
