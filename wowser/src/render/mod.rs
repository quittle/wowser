mod color;
mod css;
mod rect;
mod rendering;
mod scene;
mod style;

pub use color::*;
pub use css::*;
pub use rect::*;
pub use rendering::*;
pub use scene::*;
pub use style::*;

#[cfg(test)]
mod tests {
    use crate::{
        css::{CssDocument, CssInterpreter, CssRule, CssToken},
        html::{HtmlInterpreter, HtmlRule, HtmlToken},
        parse::{Interpreter, Lexer, Parser},
    };

    use super::*;

    fn parse_css(document: &str) -> CssDocument {
        let lexer = Lexer::new(Box::new(CssToken::Document));
        let tokens = Box::new(lexer.parse(document).expect("Failed to lex"));
        let ast = Parser {}.parse(&tokens, &CssRule::Document).expect("Failed to parse");
        let css_document = CssInterpreter {}.interpret(&ast).expect("Failed to interpret");
        css_document
    }

    fn test_css_html(css_file: &str, html_file: &str) {
        let lexer = Lexer::new(Box::new(HtmlToken::Document));
        let tokens = lexer.parse(html_file).expect("Failed to lex");
        let ast = Parser {}.parse(&tokens, &HtmlRule::Document).expect("Failed to parse");
        let html_document = HtmlInterpreter {}.interpret(&ast).expect("Failed to interpret");

        let css_document = parse_css(css_file);

        let styling = css::style_html(&html_document, &css_document);
        println!("Styling {:?}", styling);
    }

    #[test]
    fn minimal() {
        test_css_html(
            "foo { color: red; } foo .baz { height: 1; }",
            "<foo><bar class=\"baz\">text</bar></foo>",
        );
    }
}
