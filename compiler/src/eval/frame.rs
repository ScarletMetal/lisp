use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::rc::Rc;

use crate::eval::{builtins::create_builtins_map, Function};
use lisp::Value;

#[derive(Clone)]
pub struct EvalFrame {
    pub functions: HashMap<String, Rc<dyn Function>>,
    pub locals: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct EvalContext {
    frames: LinkedList<EvalFrame>,
    builtins: HashMap<String, Rc<dyn Function>>,
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
            functions: HashMap::new(),
            locals: HashMap::new(),
        }
    }
}

impl EvalContext {
    pub fn new(root: EvalFrame) -> Self {
        Self {
            frames: LinkedList::from([root]),
            builtins: create_builtins_map(),
        }
    }

    pub fn add_frame(&mut self, frame: EvalFrame) {
        self.frames.push_front(frame);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop_front();
    }

    pub fn lookup_function(&self, name: &str) -> Option<Rc<dyn Function>> {
        if let Some(func) = self.builtins.get(name) {
            return Some(func.clone());
        }

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
