use std::env;
use std::fs;
use wowser::math_parse::*;
use wowser::parse::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1 = args.get(1).expect("Document not passed in");

    let document = fs::read_to_string(arg_1).expect("Unable to read file");
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
