use super::{JsClosureContext, JsExpression, JsReference, JsStatementResult, JsValueNode};

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
                let result_bool: bool = result.map_value(|value| value.into());
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

    pub fn get_referenced_nodes(&self) -> Vec<JsValueNode> {
        match self {
            Self::Empty => vec![],
            Self::Expression(expression) => expression.get_referenced_nodes(),
            Self::VarDeclaration(reference) => reference.get_referenced_nodes(),
            Self::VariableAssignment(reference, expression) => [
                reference.get_referenced_nodes(),
                expression.get_referenced_nodes(),
            ]
            .concat(),
            Self::FunctionDeclaration(reference) => reference.get_referenced_nodes(),
            Self::Return(expression) => expression.get_referenced_nodes(),
            Self::If(expression, statements) => [
                expression.get_referenced_nodes(),
                statements
                    .iter()
                    .flat_map(|statement| statement.get_referenced_nodes())
                    .collect(),
            ]
            .concat(),
        }
    }
}
