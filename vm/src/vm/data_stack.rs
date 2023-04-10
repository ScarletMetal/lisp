use crate::bytecode::Value;

use super::execute::{ExecuteError, ExecuteResult};

#[derive(Clone, Debug)]
pub struct DataStack {
    stack: Vec<Value>,
    stack_ptr: usize,
}

impl DataStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            stack_ptr: 0,
        }
    }

    pub fn pop(&mut self) -> ExecuteResult<Value> {
        if self.stack_ptr == 0 {
            return Err(ExecuteError::EmptyDataStack);
        }

        let value = self.stack[self.stack_ptr - 1];
        self.stack_ptr -= 1;
        return Ok(value);
    }

    pub fn pop_ref(&mut self) -> ExecuteResult<usize> {
        match self.pop()? {
            Value::Reference(reference) => Ok(reference),
            _ => Err(ExecuteError::InvalidReference),
        }
    }

    pub fn pop_many(&mut self, num_params: usize) -> ExecuteResult<Vec<Value>> {
        (0..num_params)
            .map(|_| self.pop())
            .collect::<ExecuteResult<Vec<Value>>>()
    }

    pub fn push(&mut self, value: Value) -> ExecuteResult<()> {
        if self.stack.len() <= self.stack_ptr {
            self.stack.push(value);
        } else {
            self.stack[self.stack_ptr] = value;
        }

        self.stack_ptr += 1;
        Ok(())
    }

    pub fn push_many(&mut self, values: Vec<Value>) -> ExecuteResult<()> {
        values
            .iter()
            .map(|value| self.push(value.clone()))
            .collect::<ExecuteResult<()>>()
    }
}
