use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::base::{eval, EvalError, Function};
use lisp::{Expression, Value};

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

pub fn create_builtins_map() -> HashMap<String, Rc<dyn Function>> {
    HashMap::from([
        (
            String::from("+"),
            Rc::new(AddFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("-"),
            Rc::new(SubFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("*"),
            Rc::new(MulFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("/"),
            Rc::new(DivFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("setq"),
            Rc::new(SetQFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("concatenate"),
            Rc::new(ConcatenateFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("="),
            Rc::new(EqFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from(">"),
            Rc::new(GreaterFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("<"),
            Rc::new(LessFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from(">="),
            Rc::new(GreaterEqFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("<="),
            Rc::new(LessEqFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("write"),
            Rc::new(WriteFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("read"),
            Rc::new(ReadFunction {}) as Rc<dyn Function>,
        ),
        (
            String::from("progn"),
            Rc::new(ProgNFunction {}) as Rc<dyn Function>,
        ),
    ])
}
