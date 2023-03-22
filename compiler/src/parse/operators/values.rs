use lisp::{Atom, Operator, Token};

use crate::parse::base::{parse, ParseError};
use crate::parse::context::ParseContext;

pub fn parse_setq(tokens: &[Token], _context: &mut ParseContext) -> Result<Operator, ParseError> {
    match tokens {
        [Token::Atom(Atom::Name(name)), rest @ ..] => match parse(rest).next() {
            Some(Ok(expr)) => Ok(Operator::SetQ(name.clone(), expr)),
            _ => Err(ParseError::InvalidAtom),
        },
        _ => Err(ParseError::InvalidAtom),
    }
}

