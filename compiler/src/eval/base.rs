use std::rc::Rc;

use crate::eval::{frame::EvalContext, function::custom::CustomFunction, Value};
use crate::parse::ParseError;
use lisp::{Expression, Literal};

use super::invoke_function;

#[derive(Debug)]
pub enum EvalError {
    BadArguments,
    UndefinedBehaviour,
    NameNotFound(String),
    NotCallable(String),
    ParseError(ParseError),
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
                return Err(EvalError::NameNotFound(name.clone()));
            }
        }
        Expression::Literal(literal) => {
            return Ok(Value::Literal(literal.clone()));
        }
        Expression::Invoke(name, tokens) => {
            if let Some(operator) = context.lookup_operator(name) {
                return operator.eval(tokens, context);
            }
            if let Some(Value::Symbol(function)) = context.lookup_local(name) {
                return invoke_function(&*function, tokens, context);
            }

            match context.lookup_local(name) {
                Some(Value::Symbol(function)) => invoke_function(&*function, tokens, context),
                Some(_) => Err(EvalError::NotCallable(name.clone())),
                _ => Err(EvalError::NameNotFound(name.clone()))
            }
        }
        Expression::If(condition, if_case, else_or_none) => {
            if let Value::Literal(Literal::True) = eval(condition, context)? {
                eval(if_case, context)
            } else {
                if let Some(else_case) = else_or_none {
                    eval(else_case, context)
                } else {
                    Ok(Value::Literal(Literal::Nil))
                }
            }
        }
        Expression::Function(name, parameters, code) => {
            let function = CustomFunction::new(parameters.clone(), (**code).clone());
            let value = Value::Symbol(Rc::new(function));
            context.add_function(&name, &value);
            Ok(value)
        }
        Expression::Lambda(parameters, code) => Ok(Value::Symbol(Rc::new(CustomFunction::new(
            parameters.clone(),
            (**code).clone(),
        )))),
    }
}
