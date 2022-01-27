use super::{JsClosureContext, JsExpression, JsReference, JsStatementResult};

#[derive(Debug, PartialEq)]
pub enum JsStatement {
    Empty,
    Expression(JsExpression),
    VarDeclaration(JsReference),
    VariableAssignment(JsReference, JsExpression),
    FunctionDeclaration(JsReference),
    Return(JsExpression),
    If(JsExpression, Vec<JsStatement>),
}

impl JsStatement {
    pub fn run(&self, closure_context: &mut JsClosureContext) -> JsStatementResult {
        match self {
            Self::Empty => JsStatementResult::Void,
            Self::Expression(expression) => {
                JsStatementResult::Value(expression.run(closure_context))
            }
            Self::VarDeclaration(reference) => {
                let reference = closure_context.get_or_declare_reference_mut(&reference.name);
                JsStatementResult::Value(reference.value.clone())
            }
            Self::VariableAssignment(reference, expression) => {
                let value = expression.run(closure_context);
                let reference = closure_context.get_or_declare_reference_mut(&reference.name);
                reference.value = value;
                JsStatementResult::Value(reference.value.clone())
            }
            Self::FunctionDeclaration(reference) => {
                let closure_reference =
                    closure_context.get_or_declare_reference_mut(&reference.name);
                closure_reference.value = reference.value.clone();
                JsStatementResult::Value(reference.value.clone())
            }
            Self::Return(expression) => {
                JsStatementResult::ReturnValue(expression.run(closure_context))
            }
            Self::If(condition_expression, execution_statements) => {
                let result = condition_expression.run(closure_context);
                let result_bool: bool = result.as_ref().into();
                if result_bool {
                    for statement in execution_statements {
                        let result = statement.run(closure_context);
                        match result {
                            JsStatementResult::ReturnValue(_) => return result,
                            _ => closure_context.record_new_result(result),
                        };
                    }
                }
                JsStatementResult::Void
            }
        }
    }
}
