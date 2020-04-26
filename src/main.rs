#[macro_use]
extern crate lazy_static;

mod html;
mod parse;
mod math_parse;

use parse::{Lexer, Parser, ParserResult, ASTNode, Rule};
use math_parse::{MathRule, MathToken};

// use std::convert::TryFrom;
use std::borrow::Borrow;
use std::env;
use std::fs;

struct InterpreterState<T: Rule> {
    parents: Vec<Box<T>>
}

struct Interpreter {}

impl Interpreter {
    pub fn interpret<T: Rule>(&self, ast: &ParserResult<MathRule>) -> Option<i32> {
        let mut state = InterpreterState::<MathRule> {
            parents: vec!()
        };
        self._interpret::<MathRule>(&ast.node, &mut state)
    }

    fn _interpret<T: Rule>(&self, ast: &ASTNode<MathRule>, state: &mut InterpreterState<MathRule>) -> Option<i32> {
        match ast {
            ASTNode::Leaf => None,
            ASTNode::Node { rule, token, children } => {
                // let rb = MathRule::try_from(*rule);
                // let rule = rule.clone().into_any().downcast::<MathRule>();
                // let box_rule: Box<dyn Rule> = rule.into_any().downcast::<MathRule>().ok().expect("a");
                // let rule = MathRule::try_from(Box::new(*rb));
                // if let Ok(ruleb) = Ok(rule) {
                    // let ruleb = Box::from(ruleb);
                    // state.parents.push(*rule);
                    // *rule as MathRule;
                    match **rule {
                    // match *state.parents[state.parents.len() - 1] {
                        MathRule::Document => {
                            self._interpret::<T>(&children[0], state)
                        },
                        MathRule::DocumentBody => {
                            let mut result = None;
                            
                            for child in children {
                                if let Some(value) = self._interpret::<T>(child, state) {
                                    println!("Computed: {}", value);

                                    result = Some(value);
                                }
                            }
                            result
                        },
                        MathRule::Statement => {
                            // Expression + Semicolon
                            self._interpret::<T>(&children[0], state)
                        },
                        MathRule::Expression => {
                            self._interpret::<T>(&children[0], state)
                            // if let ASTNode::Node { rule, token, children } = children[0] {
                            //     if rule.eq(&MathRule::BinaryExpression) {
                            //         self._interpret(children[0], state)
                            //     }
                            // }
                            // match child {

                            // }
                            // if &children[0] 
                            // // Expression + Semicolon
                            // self._interpret::<T>(&children[0], state)
                        },
                        MathRule::BinaryExpression => {
                            println!("Rule: {:?} Child: {}", rule, children.iter().map(|c| format!("{:?}", c))
                            .fold(String::new(), |acc, c| {
                                acc + &c + "+++"
                            }));
                            let number = &children[0];
                            let operator = &children[1];
                            let expression = &children[2];
                            let v1 = self._interpret::<MathRule>(&number, state);
                            let operator: &str = if let ASTNode::Node { rule, token, children } = operator {
                                if let Some(token) = token {
                                    token.1
                                } else {
                                    panic!("Token required")
                                }
                            } else {
                                panic!("Node required")
                            };

                            let v2 = self._interpret::<MathRule>(&expression, state);

                            if let [Some(v1), Some(v2)] = [v1, v2] {
                                if operator.eq("+") {
                                    Some(v1 + v2)
                                } else {
                                    panic!("Unsupported operator")
                                }
                            } else {
                                panic!("Invalid some")
                            }
                        }
                        _ => None
                    }
                // } else {
                //     None
                // }
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
