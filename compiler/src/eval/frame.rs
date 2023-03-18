use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::rc::Rc;

use crate::eval::{
    function::builtin::create_builtin_functions_map,
    operator::{create_operators_map, Operator},
    Value,
};

#[derive(Clone)]
pub struct EvalFrame {
    pub locals: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct EvalContext {
    frames: LinkedList<EvalFrame>,
    builtins: HashMap<String, Value>,
    functions_index: HashMap<String, Value>,
    operators: HashMap<String, Rc<dyn Operator>>,
}

impl fmt::Debug for EvalFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("locals", &self.locals)
            .finish()
    }
}

impl EvalFrame {
    pub fn new(locals: HashMap<String, Value>) -> Self {
        Self { locals }
    }

    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }
}

impl EvalContext {
    pub fn new(root: EvalFrame) -> Self {
        Self {
            frames: LinkedList::from([root]),
            builtins: create_builtin_functions_map(),
            functions_index: HashMap::new(),
            operators: create_operators_map(),
        }
    }

    pub fn add_frame(&mut self, frame: EvalFrame) {
        self.frames.push_front(frame);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop_front();
    }

    pub fn add_function(&mut self, name: &String, value: &Value) {
        self.root_mut().locals.insert(name.clone(), value.clone());
        self.functions_index.insert(name.clone(), value.clone());
    }

    pub fn lookup_local(&self, name: &str) -> Option<Value> {
        if let Some(function) = self.functions_index.get(name) {
            return Some(function.clone());
        }

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

    pub fn lookup_operator(&mut self, name: &String) -> Option<Rc<dyn Operator>> {
        self.operators.get(name).map(Clone::clone)
    }

    pub fn root_mut(&mut self) -> &mut EvalFrame {
        self.frames.back_mut().unwrap()
    }

    pub fn head_mut(&mut self) -> &mut EvalFrame {
        self.frames.front_mut().unwrap()
    }
}
