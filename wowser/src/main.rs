extern crate wowser_glfw;

mod html;
mod math_parse;
mod net;
mod parse;
mod startup;
mod ui;
mod util;

use math_parse::{MathInterpreter, MathRule, MathToken};
use parse::{Interpreter, Lexer, Parser};
use ui::{Rect, Window};

use std::env;
use std::fs;
use std::thread;

fn show_ui() {
    startup::start();
    {
        let mut window = Window::new().expect("Unable to make ui.");
        window.draw_rect(&Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        });
        thread::sleep(std::time::Duration::from_millis(2000));
        window
            .resize(&Rect {
                x: 100,
                y: 100,
                width: 200,
                height: 200,
            })
            .unwrap();
        thread::sleep(std::time::Duration::from_millis(2000));
    }
    wowser_glfw::terminate();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1 = args.get(1).expect("Document not passed in");

    if arg_1 == "gui" {
        show_ui();
    } else if arg_1 == "http" {
        let verb = args.get(2).expect("Verb not passed in");
        let domain = args.get(3).expect("domain not passed in");

        let mut request = net::HttpRequest::new(net::Url::new(
            net::UrlProtocol::HTTP,
            net::UrlHost::DomainName(domain.to_string()),
            80,
            "",
            "",
            "",
        ));
        println!("Created request");
        let result = match verb.as_str() {
            "get" => futures::executor::block_on(request.get()),
            "head" => futures::executor::block_on(request.head()),
            _ => panic!("Unsupported HTTP verb {}", domain),
        };
        let response = result.expect("HttpRequest failed");
        println!("http response {:?}", response);

        match net::resolve_domain_name_to_ip(arg_1) {
            Ok(result) => println!("Result {}", result),
            Err(e) => println!("Err {:?}", e),
        }
    } else if arg_1.ends_with(".html") {
        let document = fs::read_to_string(arg_1).expect("Unable to read file");
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
    } else if arg_1.ends_with(".txt") {
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
}
