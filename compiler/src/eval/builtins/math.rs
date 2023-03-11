use lisp::{Expression, Literal};

use crate::eval::{
    builtins::eval_args, frame::EvalContext, ArgumentsSize, EvalError, Function, Value
};

pub struct AddFunction {}
pub struct SubFunction {}
pub struct MulFunction {}
pub struct DivFunction {}

impl Function for AddFunction {
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
            [Value::Literal(Literal::Number(_)), Value::Literal(Literal::Number(_)), ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val {
                        Value::Literal(Literal::Number(num)) => Ok(*num),
                        _ => Err(EvalError::UndefinedBehaviour),
                    })
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Literal(Literal::Number(
                    numbers.iter().fold(0.0, |acc, val| acc + val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for SubFunction {
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
            [Value::Literal(Literal::Number(_)), Value::Literal(Literal::Number(_)), ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val {
                        Value::Literal(Literal::Number(num)) => Ok(*num),
                        _ => Err(EvalError::UndefinedBehaviour),
                    })
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Literal(Literal::Number(
                    numbers[1..]
                        .iter()
                        .fold(*numbers.first().unwrap(), |acc, val| acc - val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for MulFunction {
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
            [Value::Literal(Literal::Number(_)), Value::Literal(Literal::Number(_)), ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val {
                        Value::Literal(Literal::Number(num)) => Ok(*num),
                        _ => Err(EvalError::UndefinedBehaviour),
                    })
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Literal(Literal::Number(
                    numbers.iter().fold(1.0, |acc, val| acc * val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}

impl Function for DivFunction {
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
            [Value::Literal(Literal::Number(_)), Value::Literal(Literal::Number(_)), ..] => {
                let numbers = args
                    .iter()
                    .map(|val| match val {
                        Value::Literal(Literal::Number(num)) => Ok(*num),
                        _ => Err(EvalError::UndefinedBehaviour),
                    })
                    .collect::<Result<Vec<f64>, EvalError>>()?;

                Ok(Value::Literal(Literal::Number(
                    numbers[1..]
                        .iter()
                        .fold(*numbers.first().unwrap(), |acc, val| acc / val),
                )))
            }
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
