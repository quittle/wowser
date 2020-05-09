extern crate wowser_glfw;

mod html;
mod math_parse;
mod net;
mod parse;
mod util;

use math_parse::{MathInterpreter, MathRule, MathToken};
use parse::{Interpreter, Lexer, Parser};

use std::env;
use std::fs;
use std::thread;

fn main() {
    wowser_glfw::glfw_init();
    wowser_glfw::glfw_create_window(800, 600, "Wowser - what a browser!");
    thread::sleep(std::time::Duration::from_millis(1000));
    wowser_glfw::glfw_terminate();

    let args: Vec<String> = env::args().collect();
    let document_file = args.get(1).expect("Document not passed in");

    match net::resolve(document_file) {
        Ok(result) => println!("Result {}", result),
        Err(e) => println!("Err {:?}", e),
    }

    if document_file.starts_with("http") {
        println!("Fetching HTML page");
    } else if document_file.ends_with(".html") {
        let document = fs::read_to_string(document_file).expect("Unable to read file");
        let lexer = Lexer::new(Box::new(html::HtmlToken::Document));
        let tokens = lexer.parse(document.as_str());
        println!("Tokens: {:?}", tokens);
        if let Some(tokens) = tokens {
            let parser = Parser {};
            let ast = parser.parse(&tokens, &html::HtmlRule::Document);
            println!("AST: {:?}", ast);
            if let Ok(ast) = ast {
                let interpreter = html::HtmlInterpreter {};
                if let Some(result) = interpreter.interpret(&ast) {
                    println!("Evaulated result {:?}", result);
                    println!("Rendered doc {}", html::stringify_node(&result));
                }
            }
        }
    } else if document_file.ends_with(".txt") {
        let document = fs::read_to_string(document_file).expect("Unable to read file");
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
