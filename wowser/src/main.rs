// #[macro_use]
// extern crate lazy_static;

mod html;
mod math_parse;
mod parse;

use math_parse::{MathInterpreter, MathRule, MathToken};
use parse::{Interpreter, Lexer, Parser};

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let document_file = args.get(1).expect("Document not passed in");
    let document = fs::read_to_string(document_file).expect("Unable to read file");

    if document_file.ends_with(".html") {
        let lexer = Lexer::new(Box::new(html::HtmlToken::Document));
        let tokens = lexer.parse(document.as_str());
        println!("Tokens: {:?}", tokens);
        if let Some(tokens) = tokens {
            let parser = Parser {};
            let ast = parser.parse(&tokens, &html::HtmlRule::Document);
            println!("AST: {:?}", ast);
        }
    } else if document_file.ends_with(".txt") {
        let lexer = Lexer::new(Box::new(MathToken::Document));
        let tokens = lexer.parse(document.as_str());
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
}
