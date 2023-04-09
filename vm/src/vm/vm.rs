use crate::bytecode::{Opcode, Value};
use crate::vm::execute::{ExecuteError, ExecuteResult};

pub const STACK_SIZE: usize = 2048;

pub struct Vm<'a> {
    pub code_chunk: &'a [Opcode],
    pub code_ptr: usize,
    pub stack_ptr: usize,
    pub stack: Vec<Value>,
}

impl<'a> Vm<'a> {
    pub fn new(code: &'a [Opcode]) -> Self {
        Self {
            code_chunk: code,
            stack: Vec::new(),
            stack_ptr: 0,
            code_ptr: 0,
        }
    }

    pub fn pop(&mut self) -> ExecuteResult<Value> {
        if self.stack_ptr == 0 {
            return Err(ExecuteError::EmptyStack);
        }

        let value = self.stack[self.stack_ptr - 1];
        self.stack_ptr -= 1;
        return Ok(value);
    }

    pub fn push(&mut self, value: Value) -> ExecuteResult<()> {
        if self.stack_ptr >= STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        if self.stack.len() <= self.stack_ptr {
            self.stack.push(value);
        } else {
            self.stack[self.stack_ptr] = value;
        }

        self.stack_ptr += 1;
        Ok(())
    }

    pub fn jump(&mut self) -> ExecuteResult<()> {
        let value = self.pop()?;
        if let Value::Reference(offset) = value {
            self.code_ptr = offset;
            return Ok(());
        } else {
            return Err(ExecuteError::InvalidValue);
        }
    }
}

