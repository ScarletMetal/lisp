use lisp::Value;

use super::eval_args;
use crate::eval::{frame::EvalContext, ArgumentsSize, EvalError, Function};

#[derive(Debug)]
pub struct EqFunction {}
#[derive(Debug)]
pub struct GreaterFunction {}
#[derive(Debug)]
pub struct LessFunction {}
#[derive(Debug)]
pub struct GreaterEqFunction {}
#[derive(Debug)]
pub struct LessEqFunction {}

impl Function for EqFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut EvalContext,
    ) -> Result<crate::lisp::Value, EvalError> {
        let args = eval_args(arguments, context)?;
        let value = args[1..].iter().all(|item| item == args.first().unwrap());
        Ok(if value { Value::True } else { Value::Nil })
    }
}

impl Function for GreaterFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(2)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right)] => Ok(if left > right {
                Value::True
            } else {
                Value::Nil
            }),
            _ => Err(EvalError::BadArguments),
        }
    }
}

impl Function for LessFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(2)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right)] => Ok(if left < right {
                Value::True
            } else {
                Value::Nil
            }),
            _ => Err(EvalError::BadArguments),
        }
    }
}

impl Function for GreaterEqFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(2)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right)] => Ok(if left >= right {
                Value::True
            } else {
                Value::Nil
            }),
            _ => Err(EvalError::BadArguments),
        }
    }
}

impl Function for LessEqFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(2)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right)] => Ok(if left <= right {
                Value::True
            } else {
                Value::Nil
            }),
            _ => Err(EvalError::BadArguments),
        }
    }
}
