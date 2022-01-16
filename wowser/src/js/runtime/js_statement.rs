use super::{JsClosure, JsExpression, JsReference, JsStatementResult};

#[derive(Debug, PartialEq)]
pub enum JsStatement {
    Empty,
    Expression(JsExpression),
    VarDeclaration(JsReference),
    VariableAssignment(JsReference, JsExpression),
    FunctionDeclaration(JsReference),
}

impl JsStatement {
    pub fn run(&self, closure: &mut JsClosure) -> JsStatementResult {
        match self {
            Self::Empty => JsStatementResult::Void,
            Self::Expression(expression) => JsStatementResult::Value(expression.run(closure)),
            Self::VarDeclaration(reference) => {
                let reference = closure.get_or_declare_reference_mut(&reference.name);
                JsStatementResult::Value(reference.value.clone())
            }
            Self::VariableAssignment(reference, expression) => {
                let value = expression.run(closure);
                let reference = closure.get_or_declare_reference_mut(&reference.name);
                reference.value = value;
                JsStatementResult::Value(reference.value.clone())
            }
            Self::FunctionDeclaration(reference) => {
                JsStatementResult::Value(reference.value.clone())
            }
        }
    }
}
