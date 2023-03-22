use lisp::{Atom, Operator, Token};

use crate::parse::base::{parse_single_expression, ParseError};
use crate::parse::context::ParseContext;

pub fn parse_function(tokens: &[Token], context: &mut ParseContext) -> Result<Operator, ParseError> {
    match tokens {
        [Token::Atom(Atom::Name(name)), Token::OpenParen, rest @ ..] => {
            let (after_parameters, parameters) = _parse_parameters(rest)?;
            let (_, code) = parse_single_expression(after_parameters, context)?;
            return Ok(Operator::Function(name.clone(), parameters, Box::new(code)))
        }
        _ => Err(ParseError::InvalidAtom)
    }
}

pub fn parse_lambda(tokens: &[Token], context: &mut ParseContext) -> Result<Operator, ParseError> {
    match tokens {
        [Token::OpenParen, rest @ ..] => {
            let (after_parameters, parameters) = _parse_parameters(rest)?;
            let (_, code) = parse_single_expression(after_parameters, context)?;
            return Ok(Operator::Lambda(parameters, Box::new(code)))
        },
        _ => Err(ParseError::InvalidAtom)
    }
}

fn _parse_parameters(tokens: &[Token]) -> Result<(&[Token], Vec<String>), ParseError> {
    let mut parameters = vec![];
    let mut temp = tokens;

    while temp.len() > 0 {
        match temp.first() {
            Some(Token::CloseParen) => {
                temp = &temp[1..]; // Skip CloseParen
                break;
            }
            Some(Token::Atom(Atom::Name(name))) => {
                temp = &temp[1..];
                parameters.push(name.clone());
            }
            Some(token) => {
                return Err(ParseError::InvalidToken(token.clone()));
            }
            _ => {
                return Err(ParseError::ExpressionNotClosed);
            }
        }
    }

    Ok((temp, parameters))
}

