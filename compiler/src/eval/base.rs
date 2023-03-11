use std::rc::Rc;

use crate::eval::Value;
use crate::eval::custom::CustomFunction;
use crate::eval::frame::{EvalContext, EvalFrame};
use lisp::{Expression, Literal};

#[derive(Debug)]
pub enum EvalError {
    BadArguments,
    UndefinedBehaviour,
    UnknownFunction(String),
    NameNotFound(String),
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
            if let Some(value) = context.lookup_variable(name) {
                return Ok(value.clone());
            } else {
                return Err(EvalError::NameNotFound(name.clone()));
            }
        }
        Expression::Literal(literal) => {
            return Ok(Value::Literal(literal.clone()));
        }
        Expression::Call(literal, children) => {
            if let Some(function) = context.lookup_function(literal) {
                if !function.get_arguments_size().contains(children.len()) {
                    return Err(EvalError::BadArguments);
                }

                context.add_frame(EvalFrame::new());
                let res = function.eval(children, context);
                context.pop_frame();
                res
            } else {
                return Err(EvalError::UnknownFunction(literal.clone()));
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
            context
                .root_mut()
                .functions
                .insert(name.clone(), Rc::new(function));
            Ok(Value::Literal(Literal::True))
        }
    }
}
