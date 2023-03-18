use crate::lisp::{Atom, Expression, Token};

#[derive(Debug)]
pub enum ParseError {
    InvalidAtom,
    ExpressionNotClosed,
    InvalidToken(Token),
}

pub struct ParseIterator<'a> {
    tokens: &'a [Token],
    should_stop: bool,
}

impl<'a> ParseIterator<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            should_stop: false,
        }
    }
}

impl<'a> Iterator for ParseIterator<'a> {
    type Item = Result<Expression, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_stop || self.tokens.len() == 0 {
            return None;
        }

        match _parse_expression(self.tokens) {
            Ok((rest, expr)) => {
                self.tokens = rest;
                return Some(Ok(expr));
            }
            Err(err) => {
                self.should_stop = true;
                return Some(Err(err));
            }
        }
    }
}

type ParseResult<'a> = (&'a [Token], Expression);

fn _get_inner_tokens<'a>(source: &'a[Token], target: &mut Vec<Token>) -> Result<&'a [Token], ParseError> {
    let mut tmp = source;
    while tmp.len() > 0 {
        match tmp.first() {
            Some(Token::CloseParen) => {
                return Ok(&tmp[1..]);
            }
            Some(Token::OpenParen) => {
                target.push(Token::OpenParen);
                tmp = _get_inner_tokens(&tmp[1..], target)?;
                target.push(Token::CloseParen);
            }
            Some(token) => {
                tmp = &tmp[1..];
                target.push(token.clone());
            }
            None => {
                break;
            }
        }
    }
    Err(ParseError::ExpressionNotClosed)
}

fn _parse_expression<'a>(tokens: &'a [Token]) -> Result<ParseResult, ParseError> {
    match tokens {
        [Token::OpenParen, Token::Atom(Atom::Name(name)), rest @ ..] => {
            let mut collected_tokens = vec![];
            let after_invoke = _get_inner_tokens(rest, &mut collected_tokens)?;
            Ok((after_invoke, Expression::Invoke(name.clone(), collected_tokens)))
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
            let (after_parameters, parameters) = _parse_parameters(rest)?;
            let (after_code, code) = _parse_expression(after_parameters)?;
            if let Some(Token::CloseParen) = after_code.first() {
                return Ok((
                    &after_code[1..],
                    Expression::Function(name.clone(), parameters, Box::new(code)),
                ));
            }
            return Err(ParseError::ExpressionNotClosed);
        }
        [Token::OpenParen, Token::Lambda, Token::OpenParen, rest @ ..] => {
            let (after_parameters, parameters) = _parse_parameters(rest)?;
            let (after_code, code) = _parse_expression(after_parameters)?;
            if let Some(Token::CloseParen) = after_code.first() {
                return Ok((
                    &after_code[1..],
                    Expression::Lambda(parameters, Box::new(code)),
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

pub fn parse(tokens: &[Token]) -> ParseIterator {
    ParseIterator::new(tokens)
}
