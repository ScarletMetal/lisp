use crate::lisp::{Atom, Expression, Token};

#[derive(Debug)]
pub enum ParseError {
    InvalidAtom,
    ExpressionNotClosed,
    InvalidToken(Token),
}

type ParseResult<'a> = (&'a [Token], Expression);

fn _parse_expression<'a>(tokens: &'a [Token]) -> Result<ParseResult, ParseError> {
    match tokens {
        [Token::OpenParen, Token::Atom(Atom::Name(name)), rest @ ..] => {
            let mut expressions = vec![];
            let mut temp = rest;

            while temp.len() > 0 {
                if let Some(Token::CloseParen) = temp.first() {
                    break;
                }

                let (after_rest, expr) = _parse_expression(temp)?;
                temp = after_rest;
                expressions.push(expr);
            }

            if let Some(Token::CloseParen) = temp.first() {
                return Ok((&temp[1..], Expression::Call(name.clone(), expressions)));
            }

            Err(ParseError::ExpressionNotClosed)
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
                return Ok((
                    &after_else_case[1..],
                    Expression::If(Box::new(condition), Box::new(if_case), else_case),
                ));
            }

            return Err(ParseError::ExpressionNotClosed);
        }
        [Token::OpenParen, Token::Defun, Token::Atom(Atom::Name(name)), Token::OpenParen, rest @ ..] =>
        {
            let mut literals = vec![];
            let mut temp = rest;

            while temp.len() > 0 {
                match temp.first() {
                    Some(Token::CloseParen) => {
                        temp = &temp[1..]; // Skip CloseParen
                        break;
                    }
                    Some(Token::Atom(Atom::Name(name))) => {
                        temp = &temp[1..];
                        literals.push(name.clone());
                    }
                    Some(token) => {
                        return Err(ParseError::InvalidToken(token.clone()));
                    }
                    _ => {
                        return Err(ParseError::ExpressionNotClosed);
                    }
                }
            }

            let (after_code, code) = _parse_expression(temp)?;
            if let Some(Token::CloseParen) = after_code.first() {
                return Ok((
                    &after_code[1..],
                    Expression::Function(name.clone(), literals, Box::new(code)),
                ));
            }
            return Err(ParseError::ExpressionNotClosed);
        }
        [Token::Atom(Atom::Literal(literal)), rest @ ..] => {
            Ok((rest, Expression::Literal(literal.clone())))
        }
        [Token::Atom(Atom::Name(name)), rest @ ..] => Ok((rest, Expression::Name(name.clone()))),
        [tok, ..] => Err(ParseError::InvalidToken(tok.clone())),
        _ => Err(ParseError::InvalidAtom),
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
