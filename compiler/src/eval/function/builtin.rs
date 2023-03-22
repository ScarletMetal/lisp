use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::{Function, Value};

pub mod io;
pub mod logic;
pub mod math;
pub mod misc;

pub use io::*;
pub use logic::*;
pub use math::*;
pub use misc::*;

pub fn create_builtin_functions_map() -> HashMap<String, Value> {
    HashMap::from([
        (
            String::from("+"),
            Value::Symbol(Rc::new(AddFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("-"),
            Value::Symbol(Rc::new(SubFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("*"),
            Value::Symbol(Rc::new(MulFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("/"),
            Value::Symbol(Rc::new(DivFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("concatenate"),
            Value::Symbol(Rc::new(ConcatenateFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("="),
            Value::Symbol(Rc::new(EqFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from(">"),
            Value::Symbol(Rc::new(GreaterFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("<"),
            Value::Symbol(Rc::new(LessFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from(">="),
            Value::Symbol(Rc::new(GreaterEqFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("<="),
            Value::Symbol(Rc::new(LessEqFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("write"),
            Value::Symbol(Rc::new(WriteFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("read"),
            Value::Symbol(Rc::new(ReadFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("and"),
            Value::Symbol(Rc::new(AndFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("or"),
            Value::Symbol(Rc::new(OrFunction {}) as Rc<dyn Function>),
        ),
        (
            String::from("not"),
            Value::Symbol(Rc::new(NotFunction {}) as Rc<dyn Function>)
    )
    ])
}
