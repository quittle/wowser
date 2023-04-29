use super::{JsClosureContext, JsExpression, JsReference, JsStatementResult, JsValueNode};

#[derive(Debug, PartialEq)]
pub enum JsStatement {
    Empty,
    Expression(JsExpression),
    VarDeclaration(JsReference),
    VariableAssignment(JsReference, JsExpression),
    FunctionDeclaration(JsReference),
    Return(JsExpression),
    Throw(JsExpression),
    /// (Condition, True Statement, False Statements)
    If(JsExpression, Vec<JsStatement>, Vec<JsStatement>),
}

impl JsStatement {
    pub fn run(&self, closure_context: &mut JsClosureContext) -> JsStatementResult {
        match self {
            Self::Empty => JsStatementResult::Void,
            Self::Expression(expression) => match expression.run(closure_context) {
                Ok(v) => JsStatementResult::Value(v),
                Err(v) => JsStatementResult::ThrowValue(v),
            },
            Self::VarDeclaration(reference) => {
                let reference = closure_context.get_or_declare_reference_mut(&reference.name);
                JsStatementResult::Value(reference.value.clone())
            }
            Self::VariableAssignment(reference, expression) => {
                match expression.run(closure_context) {
                    Ok(value) => {
                        let reference =
                            closure_context.get_or_declare_reference_mut(&reference.name);
                        reference.value = value;
                        JsStatementResult::Value(reference.value.clone())
                    }
                    Err(value) => JsStatementResult::ThrowValue(value),
                }
            }
            Self::FunctionDeclaration(reference) => {
                let closure_reference =
                    closure_context.get_or_declare_reference_mut(&reference.name);
                closure_reference.value = reference.value.clone();
                JsStatementResult::Value(reference.value.clone())
            }
            Self::Return(expression) => match expression.run(closure_context) {
                Ok(value) => JsStatementResult::ReturnValue(value),
                Err(value) => JsStatementResult::ThrowValue(value),
            },
            Self::Throw(expression) => match expression.run(closure_context) {
                Ok(value) => JsStatementResult::ThrowValue(value),
                Err(value) => JsStatementResult::ThrowValue(value),
            },
            Self::If(
                condition_expression,
                true_execution_statements,
                false_execution_statements,
            ) => match condition_expression.run(closure_context) {
                Err(value) => JsStatementResult::ThrowValue(value),
                Ok(result) => {
                    let result_bool: bool = result.map_value(|value| value.into());
                    let statements = if result_bool {
                        true_execution_statements
                    } else {
                        false_execution_statements
                    };

                    for statement in statements {
                        let result = statement.run(closure_context);
                        match result {
                            JsStatementResult::ReturnValue(_) => return result,
                            JsStatementResult::ThrowValue(_) => return result,
                            _ => closure_context.record_new_result(result),
                        };
                    }
                    JsStatementResult::Void
                }
            },
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
            Self::Throw(expression) => expression.get_referenced_nodes(),
            Self::If(expression, true_statements, false_statements) => [
                expression.get_referenced_nodes(),
                true_statements
                    .iter()
                    .chain(false_statements)
                    .flat_map(|statement| statement.get_referenced_nodes())
                    .collect(),
            ]
            .concat(),
        }
    }
}
