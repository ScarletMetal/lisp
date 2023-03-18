use std::io;

use lisp::Literal;

use crate::eval::{frame::EvalContext, ArgumentsSize, EvalError, Function, Value};

pub struct WriteFunction {}
pub struct ReadFunction {}

impl Function for WriteFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(1)
    }

    fn eval(&self, arguments: &[Value], _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
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

    fn eval(&self, _arguments: &[Value], _context: &mut EvalContext) -> Result<Value, EvalError> {
        let mut line = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut line) {
            Ok(_) => Ok(Value::Literal(Literal::String(line))),
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
