use lisp::{Atom, Expression, Literal, Value};

use super::eval_args;
use crate::eval::eval;
use crate::eval::frame::EvalContext;
use crate::eval::Function;
use crate::eval::{ArgumentsSize, EvalError};

pub struct SetQFunction {}
pub struct ConcatenateFunction {}
pub struct ProgNFunction {}

impl Function for SetQFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(2)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        match arguments {
            [Expression::Atom(Atom::Name(name)), expr] => {
                let value = eval(expr, context)?;
                context
                    .current_mut()
                    .locals
                    .insert(name.clone(), value.clone());
                Ok(value.clone())
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for ConcatenateFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(1..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;

        match &args[..] {
            [_, ..] => Ok(Value::Literal(Literal::String(args.iter().map(Value::to_string).collect()))),
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for ProgNFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(1..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        match &arguments {
            [_, ..] => {
                for (index, expression) in arguments.iter().enumerate() {
                    let value = eval(expression, context)?;
                    if index == arguments.len() - 1 {
                        return Ok(value);
                    }
                }

                return Err(EvalError::UndefinedBehaviour);
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
