use std::fmt;

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
    Value(Value),
    Name(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    Nil
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Literal(Literal)
}

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Name(String),
    Call(String, Vec<Expression>),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    Function(String, Vec<String>, Box<Expression>)
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Literal::String(str) => format!("\"{}\"", str),
            Literal::Number(num) => num.to_string(),
            Literal::True => String::from("T"),
            Literal::Nil => String::from("NIL")
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Literal(literal) => literal.fmt(f)
        }
    }
}
