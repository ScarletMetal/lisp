use std::collections::{HashMap, HashSet};

use crate::parse::operators::OperatorFactory;

use super::operators::{parse_function, parse_if, parse_lambda, parse_progn, parse_setq};

pub struct ParseContext {
    pub operators_factories: HashMap<String, OperatorFactory>,
    pub macros: HashSet<String>,
}

impl ParseContext {
    pub fn new() -> Self {
        Self {
            operators_factories: HashMap::from([
                (String::from("progn"), parse_progn as OperatorFactory),
                (String::from("setq"), parse_setq as OperatorFactory),
                (String::from("if"), parse_if as OperatorFactory),
                (String::from("defun"), parse_function as OperatorFactory),
                (String::from("lambda"), parse_lambda as OperatorFactory),
            ]),
            macros: HashSet::new(),
        }
    }
}
