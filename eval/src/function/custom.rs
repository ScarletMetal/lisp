use crate::{
    base::{ArgumentsSize, EvalError},
    frame::{EvalContext, EvalFrame},
    function::Function,
    value::Value,
};
use lisp::Expression;

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

    fn eval(&self, arguments: Vec<Value>, context: &mut EvalContext) -> Result<Value, EvalError> {
        context.add_frame(EvalFrame::new(
            self.parameter_names
                .iter()
                .map(Clone::clone)
                .zip(arguments.into_iter())
                .collect(),
        ));
        let result = eval(&self.code, context);
        context.pop_frame();
        result
    }
}
