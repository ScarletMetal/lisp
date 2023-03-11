use lisp::{Expression, Literal};

use crate::eval::{
    builtins::eval_args, frame::EvalContext, ArgumentsSize, EvalError, Function, Value,
};

pub struct AddFunction {}
pub struct SubFunction {}
pub struct MulFunction {}
pub struct DivFunction {}

impl Function for AddFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Literal(Literal::Number(first)), rest @ ..] => {
                if rest.iter().any(|value| match value {
                    Value::Literal(Literal::Number(_)) => false,
                    _ => true,
                }) {
                    return Err(EvalError::UndefinedBehaviour);
                }

                Ok(Value::Literal(Literal::Number(
                    rest.iter()
                        .filter_map(|value| match value {
                            Value::Literal(Literal::Number(n)) => Some(*n),
                            _ => None,
                        })
                        .fold(*first, |acc, val| acc + val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for SubFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Literal(Literal::Number(first)), rest @ ..] => {
                if rest.iter().any(|value| match value {
                    Value::Literal(Literal::Number(_)) => false,
                    _ => true,
                }) {
                    return Err(EvalError::UndefinedBehaviour);
                }

                Ok(Value::Literal(Literal::Number(
                    rest.iter()
                        .filter_map(|value| match value {
                            Value::Literal(Literal::Number(n)) => Some(*n),
                            _ => None,
                        })
                        .fold(*first, |acc, val| acc - val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for MulFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Literal(Literal::Number(first)), rest @ ..] => {
                if rest.iter().any(|value| match value {
                    Value::Literal(Literal::Number(_)) => false,
                    _ => true,
                }) {
                    return Err(EvalError::UndefinedBehaviour);
                }

                Ok(Value::Literal(Literal::Number(
                    rest.iter()
                        .filter_map(|value| match value {
                            Value::Literal(Literal::Number(n)) => Some(*n),
                            _ => None,
                        })
                        .fold(*first, |acc, val| acc * val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for DivFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Literal(Literal::Number(first)), rest @ ..] => {
                if rest.iter().any(|value| match value {
                    Value::Literal(Literal::Number(_)) => false,
                    _ => true,
                }) {
                    return Err(EvalError::UndefinedBehaviour);
                }

                Ok(Value::Literal(Literal::Number(
                    rest.iter()
                        .filter_map(|value| match value {
                            Value::Literal(Literal::Number(n)) => Some(*n),
                            _ => None,
                        })
                        .fold(*first, |acc, val| acc * val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
