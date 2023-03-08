use crate::eval::base::{eval, EvalError};
use crate::lisp::{Expression, Value};

use super::frame::EvalContext;

pub mod io;
pub mod logic;
pub mod math;
pub mod misc;

pub use io::*;
pub use logic::*;
pub use math::*;
pub use misc::*;

pub fn eval_args(
    arguments: &[Expression],
    context: &mut EvalContext,
) -> Result<Vec<Value>, EvalError> {
    arguments.iter().map(|arg| eval(arg, context)).collect()
}
