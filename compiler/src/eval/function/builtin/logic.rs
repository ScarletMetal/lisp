use lisp::Literal;

use crate::eval::{frame::EvalContext, ArgumentsSize, EvalError, Function, Value};

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
