use crate::eval::{
    frame::{EvalContext, EvalFrame},
    ArgumentsSize, EvalError, Function, Value,
};
use lisp::Expression;

use super::builtins::eval_args;
use super::eval;

pub struct CustomFunction {
    parameter_names: Vec<String>,
    code: Expression,
}

impl CustomFunction {
    pub fn new(parameters: Vec<String>, code: Expression) -> Self {
        Self {
            parameter_names: parameters,
            code,
        }
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
    ) -> Result<Value, EvalError> {
        let args = eval_args(arguments, context)?;

        context.add_frame(EvalFrame::new(
            self.parameter_names
                .iter()
                .map(Clone::clone)
                .zip(args.into_iter())
                .collect(),
        ));
        let result = eval(&self.code, context);
        context.pop_frame();
        result
    }
}
