use std::fmt;
use std::rc::Rc;

use crate::function::Function;
use lisp::Literal;

#[derive(Clone)]
pub enum Value {
    Literal(Literal),
    Symbol(Rc<dyn Function>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Literal(literal) => literal.fmt(f),
            Value::Symbol(_) => write!(f, "symbol"),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match &[self, other] {
            [Value::Literal(left), Value::Literal(right)] => left == right,
            _ => false,
        }
    }
}
