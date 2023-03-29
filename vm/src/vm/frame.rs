use crate::bytecode::{Opcode, Register, Value};
use crate::vm::vm::{ExecuteError, STACK_SIZE};

pub struct VmFlags {
    zero: u8,
}

pub struct VmRegisters {
    pub stack_ptr: usize,
    pub code_ptr: usize,
}

pub struct VmFrame {
    pub registers: VmRegisters,
    code: Vec<Opcode>,
    pub stack: Vec<i64>,
}

impl VmFrame {
    pub fn new(code: Vec<Opcode>) -> Self {
        Self {
            code,
            stack: Vec::new(),
            registers: VmRegisters {
                stack_ptr: 0,
                code_ptr: 0,
            },
        }
    }

    pub fn pop(&mut self) -> Result<i64, ExecuteError> {
        if self.registers.stack_ptr == 0 {
            return Err(ExecuteError::EmptyStack);
        }

        let value = self.stack[self.registers.stack_ptr - 1];
        self.registers.stack_ptr -= 1;
        return Ok(value);
    }

    fn push(&mut self, value: i64) -> Result<(), ExecuteError> {
        if self.registers.stack_ptr >= STACK_SIZE {
            return Err(ExecuteError::StackOverflow);
        }

        if self.stack.len() <= self.registers.stack_ptr {
            self.stack.push(value);
        } else {
            self.stack[self.registers.stack_ptr] = value;
        }

        self.registers.stack_ptr += 1;
        Ok(())
    }

    fn set_register(&mut self, register: &Register, value: i64) -> Result<(), ExecuteError> {
        match register {
            Register::Stack => {
                self.registers.stack_ptr = value as usize;
            }
            Register::Code => {
                self.registers.code_ptr = value as usize;
            }
        }
        Ok(())
    }

    fn retrieve_register(&mut self, register: &Register) -> Result<i64, ExecuteError> {
        match register {
            Register::Code => Ok(self.registers.code_ptr as i64),
            Register::Stack => Ok(self.registers.stack_ptr as i64),
        }
    }
}

pub fn execute(vm: &mut VmFrame) -> Result<(), ExecuteError> {
    let current = { vm.code.get(vm.registers.code_ptr).map(Clone::clone) };

    match current {
        Some(Opcode::Jump(offset)) => {
            vm.registers.code_ptr = offset;
            return Ok(());
        }
        Some(Opcode::JumpTrue(offset)) => {
            if vm.pop()? == 1 {
                vm.registers.code_ptr = offset;
            } else {
                vm.registers.code_ptr += 1;
            }
            return Ok(());
        }
        Some(Opcode::JumpFalse(offset)) => {
            if vm.pop()? == 0 {
                vm.registers.code_ptr = offset;
            } else {
                vm.registers.code_ptr += 1;
            }
            return Ok(());
        }
        _ => {}
    }

    match current {
        Some(Opcode::BinaryAdd) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            vm.push(left + right)?;
        }
        Some(Opcode::BinarySub) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            vm.push(left - right)?;
        }
        Some(Opcode::BinaryMul) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            vm.push(left * right)?;
        }
        Some(Opcode::BinaryDiv) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            vm.push(left / right)?;
        }
        Some(Opcode::Pop(register)) => {
            let value = vm.pop()?;
            vm.set_register(&register, value);
        }
        Some(Opcode::Push(value)) => match value {
            Value::Literal(number) => {
                vm.push(number)?;
            }
        },
        Some(Opcode::Compare) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            let result = if left == right { 1 } else { 0 };
            vm.push(result)?;
        }
        Some(Opcode::Noop) => {}
        Some(opcode) => {
            return Err(ExecuteError::UnhandledOpcode(opcode));
        }
        None => {
            return Err(ExecuteError::NoOpcode);
        }
    }

    vm.registers.code_ptr += 1;
    Ok(())
}
