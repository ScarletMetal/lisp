use lisp::{Expression, Token};

use crate::{
    eval::{
        base::{ArgumentsSize, EvalResult},
        eval,
        frame::EvalContext,
        EvalError, Value,
    },
    parse::parse,
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
    fn eval(&self, arguments: &[Value], context: &mut EvalContext) -> EvalResult;
}

pub fn eval_args(
    arguments: &[Expression],
    context: &mut EvalContext,
) -> Result<Vec<Value>, EvalError> {
    arguments.iter().map(|arg| eval(arg, context)).collect()
}

pub fn invoke_function(
    function: &dyn Function,
    tokens: &[Token],
    context: &mut EvalContext,
) -> EvalResult {
    let argument_expressions = parse(tokens)
        .map(|res| match res {
            Ok(expr) => Ok(expr),
            Err(err) => Err(EvalError::ParseError(err)),
        })
        .collect::<Result<Vec<Expression>, EvalError>>()?;

    if !function
        .get_arguments_size()
        .contains(argument_expressions.len())
    {
        return Err(EvalError::BadArguments);
    }

    let arguments = eval_args(&argument_expressions, context)?;

    function.eval(&arguments, context)
}
