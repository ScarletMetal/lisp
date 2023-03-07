use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Atom(Atom),
    Defun,
    If,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Literal(String),
    Value(Value)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    True,
    Nil
}

#[derive(Debug)]
pub enum Expression {
    Atom(Atom),
    Call(String, Vec<Expression>),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>)
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Value::String(str) => format!("\"{}\"", str),
            Value::Number(num) => num.to_string(),
            Value::True => String::from("T"),
            Value::Nil => String::from("NIL")
        };
        write!(f, "{}", str)
    }
}
