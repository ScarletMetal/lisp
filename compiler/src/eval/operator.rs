use lisp::{Expression, Literal, Operator};
use std::rc::Rc;

use crate::eval::{
    eval, frame::EvalContext, function::custom::CustomFunction, EvalError, EvalResult, Value,
};

fn eval_setq(name: String, expr: &Expression, context: &mut EvalContext) -> EvalResult {
    let value = eval(expr, context)?;
    context
        .head_mut()
        .locals
        .insert(name.clone(), value.clone());
    Ok(value)
}

fn eval_progn(expressions: &[Expression], context: &mut EvalContext) -> EvalResult {
    let mut result = Err(EvalError::UndefinedBehaviour);
    for expr in expressions {
        match eval(&expr, context) {
            Ok(value) => {
                result = Ok(value);
            }
            error @ _ => {
                return error;
            }
        }
    }
    result
}

fn eval_if(
    condition: &Expression,
    positive_case: &Expression,
    negative_case_or_none: Option<Box<Expression>>,
    context: &mut EvalContext,
) -> EvalResult {
    if let Value::Literal(Literal::True) = eval(condition, context)? {
        return eval(positive_case, context);
    }

    if let Some(negative_case) = negative_case_or_none {
        return eval(&*negative_case, context);
    }

    Ok(Value::Literal(Literal::Nil))
}

fn eval_function(
    name: String,
    parameters: Vec<String>,
    code: Expression,
    context: &mut EvalContext,
) -> EvalResult {
    let function = CustomFunction::new(parameters, code);
    let value = Value::Symbol(Rc::new(function));
    context.add_function(&name, &value);
    Ok(value)
}

fn eval_lambda(
    parameters: Vec<String>,
    code: Expression,
    _context: &mut EvalContext,
) -> EvalResult {
    Ok(Value::Symbol(Rc::new(CustomFunction::new(
        parameters, code,
    ))))
}

pub fn eval_opeartor(operator: Operator, context: &mut EvalContext) -> EvalResult {
    match operator {
        Operator::SetQ(name, expr) => eval_setq(name, &expr, context),
        Operator::ProgN(expressions) => eval_progn(&expressions, context),
        Operator::If(condition, positive_case, negative_case_or_none) => {
            eval_if(&*condition, &*positive_case, negative_case_or_none, context)
        }
        Operator::Function(name, parameters, code) => {
            eval_function(name, parameters, *code, context)
        }
        Operator::Lambda(parameters, code) => eval_lambda(parameters, *code, context),
    }
}
