use super::eval_args;
use crate::eval::Function;
use crate::eval::{ArgumentsSize, EvalContext, EvalError};
use crate::lisp::{Atom, Expression, Value};

pub struct F_Add {}
pub struct F_Sub {}
pub struct F_Mul {}
pub struct F_Div {}

impl Function for F_Add {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right), rest @ ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val { Value::Number(num) => Ok(*num), _ => Err(EvalError::UndefinedBehaviour) } )
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Number(numbers.iter().fold(0.0, |acc, val| acc + val)))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for F_Sub {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right), rest @ ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val { Value::Number(num) => Ok(*num), _ => Err(EvalError::UndefinedBehaviour) } )
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Number(numbers.iter().fold(0.0, |acc, val| acc - val)))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for F_Mul {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right), rest @ ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val { Value::Number(num) => Ok(*num), _ => Err(EvalError::UndefinedBehaviour) } )
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Number(numbers.iter().fold(1.0, |acc, val| acc * val)))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for F_Div {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;
        match &args[..] {
            [Value::Number(left), Value::Number(right), rest @ ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val { Value::Number(num) => Ok(*num), _ => Err(EvalError::UndefinedBehaviour) } )
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Number(numbers[1..].iter().fold(*numbers.first().unwrap(), |acc, val| acc / val)))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
