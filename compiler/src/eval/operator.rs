use lisp::{Atom, Expression, Token};
use std::{rc::Rc, collections::HashMap};

use crate::{
    eval::{eval, frame::EvalContext, EvalError, EvalResult},
    parse::parse,
};

pub trait Operator {
    fn eval(&self, tokens: &[Token], context: &mut EvalContext) -> EvalResult;
}

#[derive(Debug)]
pub struct SetQOperator {}

#[derive(Debug)]
pub struct ProgNOperator {}

impl Operator for SetQOperator {
    fn eval(&self, tokens: &[Token], context: &mut EvalContext) -> EvalResult {
        match tokens {
            [Token::Atom(Atom::Name(name)), rest @ ..] => {
                let argument_expressions = parse(rest)
                    .map(|res| match res {
                        Ok(expr) => Ok(expr),
                        Err(err) => Err(EvalError::ParseError(err)),
                    })
                    .collect::<Result<Vec<Expression>, EvalError>>()?;

                if let [expr] = &argument_expressions[..] {
                    let value = eval(expr, context)?;
                    context
                        .head_mut()
                        .locals
                        .insert(name.clone(), value.clone());
                    Ok(value)
                } else {
                    Err(EvalError::BadArguments)
                }
            }
            _ => Err(EvalError::BadArguments),
        }
    }
}

impl Operator for ProgNOperator {
    fn eval(&self, tokens: &[Token], context: &mut EvalContext) -> EvalResult {
        let mut value = Err(EvalError::BadArguments);
        for result in parse(tokens) {
            match result {
                Ok(expr) => {
                    value = eval(&expr, context);
                }
                Err(err) => { 
                    return Err(EvalError::ParseError(err));
                }
            }
        }

        value
    }
}

pub fn create_operators_map() -> HashMap<String, Rc<dyn Operator>> {
    HashMap::from([
        (String::from("setq"), Rc::new(SetQOperator {}) as Rc<dyn Operator>),
        (String::from("progn"), Rc::new(ProgNOperator {}) as Rc<dyn Operator>)
    ])
}
