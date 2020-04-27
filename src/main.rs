#[macro_use]
extern crate lazy_static;

mod html;
mod math_parse;
mod parse;

use math_parse::{MathInterpreter, MathRule, MathToken};
use parse::{Interpreter, Lexer, Parser};

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let html_document_file = args.get(1).expect("HTML document not passed in");
    let html_document = fs::read_to_string(html_document_file).expect("Unable to read file");

    let lexer = Lexer::new(Box::new(MathToken::Document));
    let tokens = lexer.parse(html_document.as_str());
    println!("Tokens: {:?}", tokens);
    if let Some(tokens) = tokens {
        let parser = Parser {};
        let ast = parser.parse(&tokens, &MathRule::Document);
        println!("AST: {:?}", ast);
        if let Ok(ast) = ast {
            let interpreter = MathInterpreter {};
            if let Some(result) = interpreter.interpret(&ast) {
                println!("Evaulated result {}", result);
            }
        }
    }
}
