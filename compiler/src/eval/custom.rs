use crate::eval::{frame::EvalContext, ArgumentsSize, Function};
use lisp::{Expression, Value};

use super::builtins::eval_args;
use super::eval;

pub struct CustomFunction {
    parameter_names: Vec<String>,
    code: Expression,
}

impl CustomFunction {
    pub fn new(parameters: Vec<String>, code: Expression) -> Self {
        Self { parameter_names: parameters, code }
    }
}

impl Function for CustomFunction {
    fn get_arguments_size(&self) -> ArgumentsSize {
        ArgumentsSize::Exact(self.parameter_names.len())
    }

    fn eval(
        &self,
        arguments: &[Expression],
        context: &mut EvalContext,
    ) -> Result<crate::lisp::Value, super::EvalError> {
        let args = eval_args(arguments, context)?;
        let current_frame = context.current_mut();
        let names_to_values: Vec<(String, Value)> = self.parameter_names.iter()
            .map(Clone::clone)
            .zip(args.iter().map(Clone::clone))
            .collect();

        for (key, value) in names_to_values {
            current_frame.locals.insert(key, value);
        }
        return eval(&self.code, context);
    }
}
