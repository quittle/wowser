mod html_document;
mod html_interpreter;
mod html_rule;
mod html_token;

pub use html_document::*;
pub use html_interpreter::HtmlInterpreter;
pub use html_rule::HtmlRule;
pub use html_token::HtmlToken;

use crate::parse::*;

pub fn parse_html(document: &str) -> Result<HtmlDocument, String> {
    let lexer = Lexer::new(Box::new(HtmlToken::Document));
    let tokens = lexer.parse(document).ok_or("Failed to lex HTML")?;
    let ast = Parser {}.parse(&tokens, &HtmlRule::Document)?;
    let document_html_node = HtmlInterpreter {}
        .interpret(&ast)
        .ok_or("Failed to interpret HTML")?;
    let html_document = HtmlDocument::from(document_html_node);
    Ok(html_document)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(document: &str) -> String {
        parse_html(document)
            .expect("Failed to parse HTML")
            .to_string()
    }

    #[test]
    fn empty_config() {
        assert_eq!(String::from("<!DOCTYPE><html></html>"), parse(""));
    }

    #[test]
    fn simple_html() {
        assert_eq!(
            String::from("<!DOCTYPE \"html\" \"PUBLIC\" \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\"><html><img src=\"foo\" preload />text<br /><b color=\"red\">here</b> as well\n</html>"),
            parse(include_str!("../../data/simple.html")));
    }
}
