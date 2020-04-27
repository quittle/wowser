#[macro_use]
extern crate lazy_static;

mod html;
mod parse;
mod math_parse;

use parse::{Lexer, Parser, ParserResult, ASTNode, Rule};
use math_parse::{MathRule, MathToken};

use std::env;
use std::fs;

struct Interpreter {}

impl Interpreter {
    pub fn interpret<T: Rule>(&self, ast: &ParserResult<MathRule>) -> Option<f32> {
        self._interpret::<MathRule>(&ast.node)
    }

    fn _interpret<T: Rule>(&self, ast: &ASTNode<MathRule>) -> Option<f32> {
        match ast {
            ASTNode::Leaf => None,
            ASTNode::Node { rule, token, children } => {
                    match **rule {
                        MathRule::Document => {
                            self._interpret::<T>(&children[0])
                        },
                        MathRule::DocumentBody => {
                            let mut result = None;
                            
                            for child in children {
                                if let Some(value) = self._interpret::<T>(child) {
                                    println!("Computed: {}", value);

                                    result = Some(value);
                                }
                            }
                            result
                        },
                        MathRule::Statement => {
                            self._interpret::<T>(&children[0])
                        },
                        MathRule::Expression => {
                            self._interpret::<T>(&children[0])
                        },
                        MathRule::BinaryExpression => {
                            let number = &children[0];
                            let operator = &children[1];
                            let expression = &children[2];
                            
                            let v1 = self._interpret::<MathRule>(&number);
                            let v2 = self._interpret::<MathRule>(&expression);

                            let operator: &str = if let ASTNode::Node { rule: _, token, children: _ } = operator {
                                if let Some(token) = token {
                                    token.1
                                } else {
                                    panic!("Token required")
                                }
                            } else {
                                panic!("Node required")
                            };


                            if let [Some(v1), Some(v2)] = [v1, v2] {
                                if operator.trim().eq("+") {
                                    Some(v1 + v2)
                                } else {
                                    panic!("Unsupported operator")
                                }
                            } else {
                                panic!("Invalid some {:?} {:?}", v1, v2)
                            }
                        },
                        MathRule::Number => {
                            if let Some(token) = token {
                                return Some(token.1.trim().parse().expect(format!("Number ({}) cannot be parsed", token.1).as_str()));
                            }
                            panic!("Invalid number")
                        },
                        _ => None
                    }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let html_document_file = args.get(1).expect("HTML document not passed in");
    let html_document = fs::read_to_string(html_document_file)
        .expect("Unable to read file");

    let lexer = Lexer::new(Box::new(MathToken::Document));
    let tokens = lexer.parse(html_document.as_str());
    println!("Tokens: {:?}", tokens);
    if let Some(tokens) = tokens {
        let parser = Parser {};
        let ast = parser.parse(&tokens, &MathRule::Document);
        println!("AST: {:?}", ast);
        if let Ok(ast) = ast {
            let interpreter = Interpreter {};
            if let Some(result) = interpreter.interpret::<MathRule>(&ast) {
                println!("Evaulated result {}", result);
            }
        }
    }
}
