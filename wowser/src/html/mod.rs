mod html_interpreter;
mod html_rule;
mod html_token;

pub use html_interpreter::{stringify_node, HtmlInterpreter, HtmlNode};
pub use html_rule::HtmlRule;
pub use html_token::HtmlToken;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::*;

    fn parse(document: &str) -> Option<String> {
        let lexer = Lexer::new(Box::new(HtmlToken::Document));
        let tokens = lexer.parse(document)?;
        let ast = Parser {}
            .parse(&tokens, &HtmlRule::Document)
            .expect("parses");
        let result = HtmlInterpreter {}.interpret(&ast)?;
        Some(stringify_node(&result))
    }

    #[test]
    fn empty_config() {
        assert_eq!(Some(String::from("<!DOCTYPE >")), parse(""));
    }

    #[test]
    fn simple_html() {
        assert_eq!(
            Some(String::from("<!DOCTYPE \"HTML\" \"PUBLIC\" \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\"><img preload=\"\" src=\"foo\" />text<br />here as well\n")),
            parse(include_str!("../../data/simple.html")));
    }
}
