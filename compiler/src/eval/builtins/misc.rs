use super::eval_args;
use crate::eval::eval;
use crate::eval::Function;
use crate::eval::{ArgumentsSize, EvalContext, EvalError};
use crate::lisp::{Atom, Expression, Value};

pub struct SetQFunction {}
pub struct ConcatenateFunction {}

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
            [Expression::Atom(Atom::Literal(literal)), expr] => {
                let value = eval(expr, context)?;
                context.values.insert(literal.clone(), value.clone());
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
            [_, ..] => Ok(Value::String(args.iter().map(Value::to_string).collect())),
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
