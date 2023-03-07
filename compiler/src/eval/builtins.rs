use crate::lisp::{Expression, Value};
use crate::eval::base::{eval, EvalError, Function};

use super::EvalContext;

pub mod logic;
pub mod math;
pub mod misc;

pub use logic::*;
pub use math::*;
pub use misc::*;

pub fn eval_args(arguments: &[Expression], context: &mut EvalContext) -> Result<Vec<Value>, EvalError> {
    arguments
        .iter()
        .map(|arg| eval(arg, context))
        .collect()
}

