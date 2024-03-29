use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Atom(Atom),
    Defun,
    Lambda,
    Backtick,
    Comma,
    If,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Literal(Literal),
    Name(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    Nil
}

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Name(String),
    Invoke(String, Vec<Token>),
    Call(String, Vec<Expression>),
    Operator(Box<Operator>)
}

#[derive(Clone, Debug)]
pub enum Operator {
    SetQ(String, Expression),
    ProgN(Vec<Expression>),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    Function(String, Vec<String>, Box<Expression>),
    Lambda(Vec<String>, Box<Expression>),
    // Let(Vec<(String, Expression)>),
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

