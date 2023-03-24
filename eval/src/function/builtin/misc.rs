use lisp::Literal;

use crate::{
    base::{ArgumentsSize, EvalError},
    frame::EvalContext,
    function::Function,
    value::Value,
};

pub struct ConcatenateFunction {}

impl Function for ConcatenateFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(1..)
    }

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
            [_, ..] => Ok(Value::Literal(Literal::String(
                arguments.iter().map(Value::to_string).collect(),
            ))),
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
