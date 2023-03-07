use crate::lisp::{Atom, Expression, Token};
use crate::scan::Scanner;

#[derive(Debug)]
pub enum ParseError {
    InvalidAtom,
    ExpressionNotClosed,
    InvalidToken(Token),
}

type ParseResult<'a> = (&'a [Token], Expression);

fn _parse_expression<'a>(tokens: &'a [Token]) -> Result<ParseResult, ParseError> {
    match tokens {
        [Token::OpenParen, Token::Atom(Atom::Literal(literal)), rest @ ..] => {
            let mut expressions = vec![];
            let mut temp = rest;

            while temp.len() > 0 {
                if let Some(Token::CloseParen) = temp.first() {
                    temp = &temp[1..]; // Skip CloseParen
                    break;
                }

                let (after_rest, expr) = _parse_expression(temp)?;
                temp = after_rest;
                expressions.push(expr);
            }

            let res = Ok((temp, Expression::Call(literal.clone(), expressions)));
            return res;
        }
        [Token::OpenParen, Token::If, rest @ ..] => {
            let (after_condition, condition) = _parse_expression(rest)?;
            let (after_if_case, if_case) = _parse_expression(after_condition)?;
            let (after_else_case, else_case) = {
                if let Some(Token::CloseParen) = after_if_case.first() {
                    (after_if_case, None)
                } else {
                    let (after_else_case, else_case) = _parse_expression(after_if_case)?;
                    (after_else_case, Some(Box::new(else_case)))
                }
            };

            if let Some(Token::CloseParen) = after_else_case.first() {
                return Ok((&after_else_case[1..], Expression::If(Box::new(condition), Box::new(if_case), else_case)));
            }

            return Err(ParseError::ExpressionNotClosed);
        }
        [Token::Atom(atom), rest @ ..] => {
            return Ok((rest, Expression::Atom(atom.clone())));
        }
        [tok, ..] => {
            return Err(ParseError::InvalidToken(tok.clone()));
        }
        _ => {
            return Err(ParseError::InvalidAtom);
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expression>, ParseError> {
    let mut expressions = vec![];

    let mut temp: &[Token] = &tokens;

    while temp.len() > 0 {
        let (rest, result) = _parse_expression(temp)?;
        temp = rest;
        expressions.push(result);
    }

    Ok(expressions)
}
