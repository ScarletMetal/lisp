use crate::bytecode::{Opcode, Value};

use super::call_stack::{CallStack, CallStackFrame};
use super::code::{CodePtr, CodeVector};
use super::data_stack::DataStack;
use super::execute::{ExecuteError, ExecuteResult};

#[derive(Debug)]
pub struct Vm {
    pub call_stack: CallStack,
    pub data_stack: DataStack,
    pub context: CodeVector,
}

impl Vm {
    pub fn new(context: CodeVector) -> Self {
        Self {
            call_stack: CallStack::new(),
            data_stack: DataStack::new(),
            context,
        }
    }

    pub fn call(&mut self, chunk_id: usize, num_params: usize) -> ExecuteResult<()> {
        if !self.context.has_chunk(chunk_id) {
            return Err(ExecuteError::InvalidReference);
        }

        let params = self.data_stack.pop_many(num_params)?;

        self.call_stack
            .push(CallStackFrame::new(CodePtr::new(chunk_id, 0), params));

        Ok(())
    }

    pub fn ret(&mut self, num_ret: usize) -> ExecuteResult<()> {
        if self.context.is_empty() {
            return Err(ExecuteError::EmptyCallStack);
        }

        let return_values = self.data_stack.pop_many(num_ret)?;
        self.call_stack.pop()?;
        self.data_stack.push_many(return_values)?;
        Ok(())
    }

    fn current_frame(&self) -> ExecuteResult<&CallStackFrame> {
        self.call_stack.peek()
    }

    pub fn jump(&mut self, offset: usize) -> ExecuteResult<()> {
        self.call_stack
            .peek_mut()?
            .get_code_ptr_mut()
            .set_code_ptr(offset);
        Ok(())
    }

    pub fn pop(&mut self) -> ExecuteResult<Value> {
        self.data_stack.pop()
    }

    pub fn pop_ref(&mut self) -> ExecuteResult<usize> {
        self.data_stack.pop_ref()
    }

    pub fn push(&mut self, value: Value) -> ExecuteResult<()> {
        self.data_stack.push(value)
    }

    pub fn get_current_opcode(&self) -> ExecuteResult<&Opcode> {
        let code_ptr = self.call_stack.peek()?.get_code_ptr();

        self.context
            .get_current_opcode(code_ptr.get_chunk_id(), code_ptr.get_code_ptr())
            .ok_or(ExecuteError::NoOpcode)
    }

    pub fn step(&mut self) -> ExecuteResult<()> {
        let code = self.call_stack.peek_mut()?.get_code_ptr_mut();
        code.set_code_ptr(code.get_code_ptr() + 1);
        Ok(())
    }

    pub fn get_locals(&self) -> ExecuteResult<&[Value]> {
        self.current_frame().map(|frame| frame.locals())
    }
}
