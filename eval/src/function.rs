use lisp::Expression;

use crate::{
    base::{eval, ArgumentsSize, EvalError, EvalResult},
    frame::EvalContext,
    value::Value,
};

pub mod builtin;
pub mod custom;

impl ArgumentsSize {
    pub fn contains(&self, value: usize) -> bool {
        match self {
            ArgumentsSize::Exact(exact_size) => value == *exact_size,
            ArgumentsSize::Range(range) => range.contains(&value),
        }
    }
}

pub trait Function {
    fn get_arguments_size(&self) -> ArgumentsSize;
    fn eval(&self, arguments: Vec<Value>, context: &mut EvalContext) -> EvalResult;
}

pub fn eval_args(
    arguments: &[Expression],
    context: &mut EvalContext,
) -> Result<Vec<Value>, EvalError> {
    arguments.iter().map(|arg| eval(arg, context)).collect()
}

pub fn invoke_function(
    function: &dyn Function,
    expressions: &[Expression],
    context: &mut EvalContext,
) -> EvalResult {
    if !function.get_arguments_size().contains(expressions.len()) {
        return Err(EvalError::BadArguments);
    }

    let arguments = eval_args(&expressions, context)?;

    function.eval(arguments, context)
}
