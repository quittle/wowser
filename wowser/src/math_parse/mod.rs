mod math_interpreter;
mod math_rule;
mod math_token;

pub use math_interpreter::MathInterpreter;
pub use math_rule::MathRule;
pub use math_token::MathToken;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::*;

    fn parse(document: &str) -> Option<f32> {
        let lexer = Lexer::new(Box::new(MathToken::Document));
        let tokens = lexer.parse(document).expect("Failed to lex");
        let ast = Parser {}.parse(&tokens, &MathRule::Document).expect("Failed to parse");
        MathInterpreter {}.interpret(&ast)
    }

    #[test]
    fn empty_config() {
        assert_eq!(None, parse(""));
    }

    #[test]
    fn simple_number() {
        assert_eq!(Some(1f32), parse("1;"));
    }

    #[test]
    fn last_expression_wins() {
        assert_eq!(Some(3f32), parse("1; 2; 3;"));
    }

    #[test]
    fn complex_expression_and_spacing() {
        assert_eq!(Some(1.2), parse(" 0 ;-1+-1;    1 + 2.3+-2.1 ; "));
    }
}
