use crate::eval::{ArgumentsSize, EvalError, Function};
use crate::lisp::Value;
use std::io;

use super::eval_args;

pub struct WriteFunction {}
pub struct ReadFunction {}

impl Function for WriteFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(1)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut crate::eval::EvalContext,
    ) -> Result<Value, crate::eval::EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [val] => {
                println!("{}", val);
                Ok(val.clone())
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for ReadFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(0)
    }

    fn eval(
        &self,
        _arguments: &[crate::lisp::Expression],
        _context: &mut crate::eval::EvalContext,
    ) -> Result<Value, EvalError> {
        let mut line = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut line) {
            Ok(_) => Ok(Value::String(line)),
            _ => Err(EvalError::UndefinedBehaviour)
        }

    }
}
