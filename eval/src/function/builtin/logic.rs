use lisp::Literal;

use crate::{
    base::{EvalError, EvalResult},
    frame::EvalContext,
    function::{ArgumentsSize, Function},
    value::Value,
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

#[derive(Debug)]
pub struct AndFunction {}
#[derive(Debug)]
pub struct OrFunction {}
#[derive(Debug)]
pub struct NotFunction {}

impl Function for EqFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        let value = arguments[1..]
            .iter()
            .all(|item| item == arguments.first().unwrap());
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

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
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

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
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

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
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

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> Result<Value, EvalError> {
        match &arguments[..] {
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

impl Function for AndFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> EvalResult {
        if arguments
            .into_iter()
            .any(|arg| arg == Value::Literal(Literal::Nil))
        {
            Ok(Value::Literal(Literal::Nil))
        } else {
            Ok(Value::Literal(Literal::True))
        }
    }
}

impl Function for OrFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> EvalResult {
        if arguments
            .into_iter()
            .any(|arg| arg == Value::Literal(Literal::True))
        {
            Ok(Value::Literal(Literal::True))
        } else {
            Ok(Value::Literal(Literal::Nil))
        }
    }
}

impl Function for NotFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(1)
    }

    fn eval(&self, arguments: Vec<Value>, _context: &mut EvalContext) -> EvalResult {
        match &arguments[..] {
            [Value::Literal(Literal::Nil)] => Ok(Value::Literal(Literal::True)),
            [_] => Ok(Value::Literal(Literal::Nil)),
            _ => Err(EvalError::UndefinedBehaviour),
        }
    }
}
