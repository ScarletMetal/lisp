use crate::{frame::EvalContext, value::Value};
use lisp::Expression;

use crate::{
    function::{invoke_function},
    operator::eval_operator,
};

#[derive(Debug)]
pub enum EvalError {
    BadArguments,
    UndefinedBehaviour,
    NameNotFound(String),
    NotCallable(String),
}

#[derive(Debug)]
pub enum ArgumentsSize {
    Exact(usize),
    Range(std::ops::RangeFrom<usize>),
}

pub type EvalResult = Result<Value, EvalError>;

pub fn eval(expr: &Expression, context: &mut EvalContext) -> EvalResult {
    match expr {
        Expression::Name(name) => {
            if let Some(value) = context.lookup_local(name) {
                return Ok(value.clone());
            } else {
                return Err(EvalError::NameNotFound(String::from(name)));
            }
        }
        Expression::Literal(literal) => {
            return Ok(Value::Literal(literal.clone()));
        }
        Expression::Call(name, expressions) => match context.lookup_local(name) {
            Some(Value::Symbol(function)) => invoke_function(&*function, expressions, context),
            Some(_) => Err(EvalError::NotCallable(String::from(name))),
            _ => Err(EvalError::NameNotFound(String::from(name))),
        },
        Expression::Operator(operator) => eval_operator(*operator.clone(), context),
        _ => Err(EvalError::UndefinedBehaviour),
    }
}
