use std::collections::VecDeque;

use crate::bytecode::Value;

use super::code::CodePtr;
use super::execute::{ExecuteError, ExecuteResult};

#[derive(Clone, Debug)]
pub struct CallStackFrame {
    locals: Vec<Value>,

    pub code_ptr: CodePtr,
}

#[derive(Clone, Debug)]
pub struct CallStack {
    frames: VecDeque<CallStackFrame>,
}

impl CallStackFrame {
    pub fn new(code_ptr: CodePtr, locals: Vec<Value>) -> Self {
        Self { code_ptr, locals }
    }

    pub fn locals(&self) -> &[Value] {
        &self.locals[..]
    }

    pub fn get_code_ptr(&self) -> &CodePtr {
        &self.code_ptr
    }

    pub fn get_code_ptr_mut(&mut self) -> &mut CodePtr {
        &mut self.code_ptr
    }
}

impl CallStack {
    pub fn new() -> Self {
        CallStack {
            frames: VecDeque::from([CallStackFrame::new(CodePtr::new(0, 0), Vec::new())]),
        }
    }

    pub fn peek(&self) -> ExecuteResult<&CallStackFrame> {
        self.frames.front().ok_or(ExecuteError::EmptyCallStack)
    }

    pub fn peek_mut(&mut self) -> ExecuteResult<&mut CallStackFrame> {
        self.frames.front_mut().ok_or(ExecuteError::EmptyCallStack)
    }

    pub fn push(&mut self, frame: CallStackFrame) {
        self.frames.push_front(frame)
    }

    pub fn pop(&mut self) -> ExecuteResult<CallStackFrame> {
        self.frames.pop_front().ok_or(ExecuteError::EmptyCallStack)
    }
}
