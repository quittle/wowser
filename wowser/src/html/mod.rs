mod html_document;
mod html_interpreter;
mod html_rule;
mod html_token;

pub use html_document::*;
pub use html_interpreter::HtmlInterpreter;
pub use html_rule::HtmlRule;
pub use html_token::HtmlToken;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::*;

    fn parse(document: &str) -> Option<String> {
        let lexer = Lexer::new(Box::new(HtmlToken::Document));
        let tokens = lexer.parse(document)?;
        let ast = Parser {}.parse(&tokens, &HtmlRule::Document).ok()?;
        let result = HtmlInterpreter {}.interpret(&ast)?;
        Some(result.to_string())
    }

    #[test]
    fn empty_config() {
        assert_eq!(Some(String::from("<!DOCTYPE >")), parse(""));
    }

    #[test]
    fn simple_html() {
        assert_eq!(
            Some(String::from("<!DOCTYPE \"html\" \"PUBLIC\" \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\"><img src=\"foo\" preload />text<br /><b color=\"red\">here</b> as well\n")),
            parse(include_str!("../../data/simple.html")));
    }
}
