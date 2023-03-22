use lisp::{Atom, Expression, Token};

use crate::parse::context::ParseContext;

type ParseResult<'a> = (&'a [Token], Expression);

#[derive(Debug)]
pub enum ParseError {
    InvalidAtom,
    ExpressionNotClosed,
    InvalidToken(Token),
}

enum InitialParseResult {
    Expression(Expression),
    Invoke(String, Vec<Token>),
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
        let mut context = ParseContext::new();
        if self.should_stop || self.tokens.len() == 0 {
            return None;
        }

        match parse_single_expression(self.tokens, &mut context) {
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

fn _get_inner_tokens<'a>(
    source: &'a [Token],
    target: &mut Vec<Token>,
) -> Result<&'a [Token], ParseError> {
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

pub fn parse_single_expression<'a>(
    tokens: &'a [Token],
    context: &mut ParseContext,
) -> Result<ParseResult<'a>, ParseError> {
    let initial_parse_result = _initial_parse_expression(tokens, context)?;

    match initial_parse_result {
        (rest, InitialParseResult::Expression(expr)) => Ok((rest, expr)),
        (rest, InitialParseResult::Invoke(name, tokens)) => {
            match context.operators_factories.get(&name) {
                Some(factory) => {
                    let operator = factory(&tokens, context)?;
                    Ok((rest, Expression::Operator(Box::new(operator))))
                }
                None => {
                    let expressions =
                        parse(&tokens).collect::<Result<Vec<Expression>, ParseError>>()?;
                    Ok((rest, Expression::Call(name.clone(), expressions)))
                }
            }
        }
    }
}

fn _initial_parse_expression<'a>(
    tokens: &'a [Token],
    _context: &mut ParseContext,
) -> Result<(&'a [Token], InitialParseResult), ParseError> {
    match tokens {
        [Token::OpenParen, Token::Atom(Atom::Name(name)), rest @ ..] => {
            let mut collected_tokens = vec![];
            let after_invoke = _get_inner_tokens(rest, &mut collected_tokens)?;
            Ok((
                after_invoke,
                InitialParseResult::Invoke(name.clone(), collected_tokens),
            ))
        }
        [Token::Atom(Atom::Literal(literal)), rest @ ..] => Ok((
            rest,
            InitialParseResult::Expression(Expression::Literal(literal.clone())),
        )),
        [Token::Atom(Atom::Name(name)), rest @ ..] => Ok((
            rest,
            InitialParseResult::Expression(Expression::Name(name.clone())),
        )),
        [tok, ..] => Err(ParseError::InvalidToken(tok.clone())),
        _ => Err(ParseError::InvalidAtom),
    }
}

pub fn parse(tokens: &[Token]) -> ParseIterator {
    ParseIterator::new(tokens)
}
