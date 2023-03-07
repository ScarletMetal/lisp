use super::eval_args;
use crate::{
    eval::{EvalError, ArgumentsSize, Function},
    lisp::Value,
};

#[derive(Debug)]
pub struct F_Eq {}

impl Function for F_Eq {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Range(2..)
    }

    fn eval(
        &self,
        arguments: &[crate::lisp::Expression],
        context: &mut crate::eval::EvalContext,
    ) -> Result<crate::lisp::Value, EvalError> {
        let args = eval_args(arguments, context)?;
        let value = args[1..].iter().all(|item| item == args.first().unwrap());
        Ok(if value { Value::True } else { Value::Nil })
    }
}
