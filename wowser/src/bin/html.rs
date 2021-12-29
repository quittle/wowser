use wowser::parse::{Interpreter, Lexer, Parser};
use wowser::{html, log};

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1 = args.get(1).expect("Document not passed in");

    let document = fs::read_to_string(arg_1).expect("Unable to read file");
    let lexer = Lexer::new(html::HtmlToken::Document);
    let tokens = lexer.parse(document.as_str());
    log!(INFO: "Tokens:", tokens);
    if let Some(tokens) = tokens {
        let parser = Parser {};
        let ast = parser.parse(&tokens, &html::HtmlRule::Document);
        log!(INFO: "AST:", ast);
        if let Ok(ast) = ast {
            let interpreter = html::HtmlInterpreter {};
            if let Some(result) = interpreter.interpret(&ast) {
                log!(INFO: "Evaulated result", result);
                log!(INFO: "Rendered doc", result.to_string());
            }
        }
    }
}
