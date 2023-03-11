use std::collections::{HashMap, LinkedList};
use std::fmt;

use crate::eval::{builtins::create_builtins_map, Value};

#[derive(Clone)]
pub struct EvalFrame {
    pub locals: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct EvalContext {
    frames: LinkedList<EvalFrame>,
    builtins: HashMap<String, Value>,
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
            locals: HashMap::new()
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

    pub fn lookup_local(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.builtins.get(name) {
            return Some(value.clone());
        }

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
