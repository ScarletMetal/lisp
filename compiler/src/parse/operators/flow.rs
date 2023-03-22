use lisp::{Expression, Operator, Token};

use crate::parse::base::{parse, parse_single_expression, ParseError};
use crate::parse::context::ParseContext;

pub fn parse_progn(tokens: &[Token], _context: &mut ParseContext) -> Result<Operator, ParseError> {
    let expressions = parse(tokens).collect::<Result<Vec<Expression>, ParseError>>()?;
    Ok(Operator::ProgN(expressions))
}

pub fn parse_if(tokens: &[Token], context: &mut ParseContext) -> Result<Operator, ParseError> {
    let (after_condition, condition) = parse_single_expression(tokens, context)?;
    let (after_positive_case, positive_case) = parse_single_expression(after_condition, context)?;
    let negative_case = {
        if let Some(Token::CloseParen) = after_positive_case.first() {
            None
        } else {
            let (_, else_case) = parse_single_expression(after_positive_case, context)?;
            Some(Box::new(else_case))
        }
    };

    return Ok(Operator::If(
        Box::new(condition),
        Box::new(positive_case),
        negative_case,
    ));
}
