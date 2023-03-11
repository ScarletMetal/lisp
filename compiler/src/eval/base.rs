use std::rc::Rc;

use crate::eval::custom::CustomFunction;
use crate::eval::frame::EvalContext;
use crate::eval::Value;
use lisp::{Expression, Literal};

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
    fn eval(&self, arguments: &[Expression], context: &mut EvalContext)
        -> Result<Value, EvalError>;
}

pub fn eval(expr: &Expression, context: &mut EvalContext) -> Result<Value, EvalError> {
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
        Expression::Call(name, children) => match context.lookup_local(name) {
            Some(Value::Symbol(function)) => {
                if !function.get_arguments_size().contains(children.len()) {
                    return Err(EvalError::BadArguments);
                }

                function.eval(children, context)
            }
            Some(Value::Literal(_)) => Err(EvalError::NotCallable(name.clone())),
            None => Err(EvalError::NameNotFound(name.clone())),
        },
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
