#[macro_use]
extern crate lazy_static;

mod html;
mod parse;
mod math_parse;

use parse::{Lexer, Parser, ParserResult, ASTNode};
use math_parse::{MathRule, MathToken};

use std::env;
use std::fs;

struct Interpreter {}

impl Interpreter {
    fn interpret(&self, ast: ParserResult) -> Option<i32> {
        match ast.node {
            ASTNode::Leaf => None,
            ASTNode::Node { rule, token, children } => {
                match *rule {
                    // MathRule::Document { Document } => {
                    //     self.interpret(children[0])
                    // },
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
            let result = interpreter.interpret(ast);
        }
    }
}
