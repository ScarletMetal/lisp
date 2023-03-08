use crate::lisp::{Expression, Value};
use crate::eval::base::{eval, EvalError};

use super::EvalContext;

pub mod logic;
pub mod math;
pub mod misc;
pub mod io;

pub use logic::*;
pub use math::*;
pub use misc::*;
pub use io::*;

pub fn eval_args(arguments: &[Expression], context: &mut EvalContext) -> Result<Vec<Value>, EvalError> {
    arguments
        .iter()
        .map(|arg| eval(arg, context))
        .collect()
}

