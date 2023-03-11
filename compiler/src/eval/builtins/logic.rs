use lisp::Literal;

use crate::eval::{
    builtins::eval_args, frame::EvalContext, ArgumentsSize, EvalError, Function, Value,
};

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
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        let value = args[1..].iter().all(|item| item == args.first().unwrap());
        Ok(if value {
            Value::Literal(Literal::True)
        } else {
            Value::Literal(Literal::Nil)
        })
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
            [Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))] => {
                Ok(if left > right {
                    Value::Literal(Literal::True)
                } else {
                    Value::Literal(Literal::Nil)
                })
            }
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
            [Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))] => {
                Ok(if left < right {
                    Value::Literal(Literal::True)
                } else {
                    Value::Literal(Literal::Nil)
                })
            }
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
            [Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))] => {
                Ok(if left >= right {
                    Value::Literal(Literal::True)
                } else {
                    Value::Literal(Literal::Nil)
                })
            }
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
            [Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))] => {
                Ok(if left <= right {
                    Value::Literal(Literal::True)
                } else {
                    Value::Literal(Literal::Nil)
                })
            }
            _ => Err(EvalError::BadArguments),
        }
    }
}
