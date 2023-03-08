use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::rc::Rc;

use crate::{eval::Function, lisp::Value};

#[derive(Clone)]
pub struct EvalFrame {
    pub functions: HashMap<String, Rc<dyn Function>>,
    pub locals: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct EvalContext {
    frames: LinkedList<EvalFrame>,
}

impl fmt::Debug for EvalFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("locals", &self.locals)
            .finish()
    }
}

impl EvalFrame {
    pub fn new() -> Self {
        Self {
            functions: HashMap::from([
                (
                    String::from("+"),
                    Rc::new(super::builtins::AddFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("-"),
                    Rc::new(super::builtins::SubFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("*"),
                    Rc::new(super::builtins::MulFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("/"),
                    Rc::new(super::builtins::DivFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("setq"),
                    Rc::new(super::builtins::SetQFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("concatenate"),
                    Rc::new(super::builtins::ConcatenateFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("="),
                    Rc::new(super::builtins::EqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from(">"),
                    Rc::new(super::builtins::GreaterFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("<"),
                    Rc::new(super::builtins::LessFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from(">="),
                    Rc::new(super::builtins::GreaterEqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("<="),
                    Rc::new(super::builtins::LessEqFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("write"),
                    Rc::new(super::builtins::WriteFunction {}) as Rc<dyn Function>,
                ),
                (
                    String::from("read"),
                    Rc::new(super::builtins::ReadFunction {}) as Rc<dyn Function>,
                ),
            ]),
            locals: HashMap::new(),
        }
    }
}

impl EvalContext {
    pub fn new(root: EvalFrame) -> Self {
        Self {
            frames: LinkedList::from([root]),
        }
    }

    pub fn add_frame(&mut self, frame: EvalFrame) {
        self.frames.push_front(frame);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop_front();
    }

    pub fn lookup_function(&self, name: &str) -> Option<Rc<dyn Function>> {
        for frame in self.frames.iter() {
            if let Some(func) = frame.functions.get(name) {
                return Some(func.clone());
            }
        }

        None
    }

    pub fn lookup_variable(&self, name: &str) -> Option<Value> {
        for frame in self.frames.iter() {
            if let Some(var) = frame.locals.get(name) {
                let val = Some(var.clone());
                return val;
            }
        }

        None
    }

    pub fn root_mut(&mut self) -> &mut EvalFrame {
        self.frames.back_mut().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut EvalFrame {
        self.frames.front_mut().unwrap()
    }
}
