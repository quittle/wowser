use super::super::parse::*;
use super::math_rule::MathRule;

pub struct MathInterpreter {}

impl Interpreter<'_, MathRule> for MathInterpreter {
    type Result = f32;

    fn on_node(&self, ast: &ASTNode<MathRule>) -> Option<f32> {
        let ASTNode {
            rule,
            token,
            children,
            ..
        } = ast;

        match rule {
            MathRule::Document => self.on_node(&children[0]),
            MathRule::DocumentBody => {
                let mut result = None;

                for child in children {
                    if let Some(value) = self.on_node(child) {
                        log!(INFO: "Computed:", value);

                        result = Some(value);
                    }
                }
                result
            }
            MathRule::Statement => self.on_node(&children[0]),
            MathRule::Expression => self.on_node(&children[0]),
            MathRule::BinaryExpression => {
                let number = &children[0];
                let operator = &children[1];
                let expression = &children[2];

                let v1 = self.on_node(number);
                let v2 = self.on_node(expression);

                let operator: &str = if let Some(token) = operator.token {
                    token.literal
                } else {
                    panic!("Token required")
                };

                if let [Some(v1), Some(v2)] = [v1, v2] {
                    if operator.eq("+") {
                        Some(v1 + v2)
                    } else {
                        panic!("Unsupported operator")
                    }
                } else {
                    panic!("Invalid some {:?} {:?}", v1, v2)
                }
            }
            MathRule::Number => {
                if let Some(token) = token {
                    return Some(token.literal.parse().unwrap_or_else(|_| {
                        panic!("Number ({}) cannot be parsed", token.literal)
                    }));
                }
                panic!("Invalid number")
            }
            MathRule::BinaryOperator | MathRule::Semicolon | MathRule::Terminator => None,
        }
    }
}
