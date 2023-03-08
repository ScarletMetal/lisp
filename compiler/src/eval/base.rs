use std::collections::HashMap;
use std::rc::Rc;

use crate::lisp::{Atom, Expression, Value};

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

pub struct EvalContext {
    pub functions: HashMap<String, Rc<dyn Function>>,
    pub values: HashMap<String, Value>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            functions: HashMap::from([
                (
                    String::from("+"),
                    Rc::new(super::builtins::AddFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("-"),
                    Rc::new(super::builtins::SubFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("*"),
                    Rc::new(super::builtins::MulFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("/"),
                    Rc::new(super::builtins::DivFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("setq"),
                    Rc::new(super::builtins::SetQFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("concatenate"),
                    Rc::new(super::builtins::ConcatenateFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("="),
                    Rc::new(super::builtins::EqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from(">"),
                    Rc::new(super::builtins::GreaterFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("<"),
                    Rc::new(super::builtins::LessFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from(">="),
                    Rc::new(super::builtins::GreaterEqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("<="),
                    Rc::new(super::builtins::LessEqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("write"),
                    Rc::new(super::builtins::WriteFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("read"),
                    Rc::new(super::builtins::ReadFunction {}) as Rc<dyn Function>,
                ),
            ]),
            values: HashMap::new(),
        }
    }
}

pub fn eval(expr: &Expression, context: &mut EvalContext) -> Result<Value, EvalError> {
    match expr {
        Expression::Atom(Atom::Literal(literal)) => {
            if let Some(value) = context.values.get(literal) {
                return Ok(value.clone());
            } else {
                return Err(EvalError::NameNotFound(literal.clone()));
            }
        }
        Expression::Atom(Atom::Value(value)) => {
            return Ok(value.clone());
        }
        Expression::Call(literal, children) => {
            if context.functions.contains_key(literal) {
                let function = context.functions.get(literal).unwrap().clone();
                if !function.get_arguments_size().contains(children.len()) {
                    return Err(EvalError::BadArguments);
                }
                return function.eval(&children, context);
            } else {
                return Err(EvalError::UnknownFunction(literal.clone()));
            }
        }
        Expression::If(condition, if_case, else_or_none) => {
            if let Value::True = eval(condition, context)? {
                eval(if_case, context)
            } else {
                if let Some(else_case) = else_or_none {
                    eval(else_case, context)
                } else {
                    Ok(Value::Nil)
                }
            }
        }
    }
}
