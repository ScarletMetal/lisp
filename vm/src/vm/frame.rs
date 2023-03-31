use crate::bytecode::{Boolean, Opcode, Value};
use crate::vm::vm::{ExecuteError, STACK_SIZE};

pub struct VmRegisters {
    pub stack_ptr: usize,
    pub code_ptr: usize,
}

pub struct Vm {
    pub registers: VmRegisters,
    code: Vec<Opcode>,
    pub stack: Vec<Value>,
}

impl Vm {
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

    pub fn pop(&mut self) -> Result<Value, ExecuteError> {
        if self.registers.stack_ptr == 0 {
            return Err(ExecuteError::EmptyStack);
        }

        let value = self.stack[self.registers.stack_ptr - 1];
        self.registers.stack_ptr -= 1;
        return Ok(value);
    }

    fn push(&mut self, value: Value) -> Result<(), ExecuteError> {
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
}

pub fn execute(vm: &mut Vm) -> Result<(), ExecuteError> {
    let current = { vm.code.get(vm.registers.code_ptr).map(Clone::clone) };

    match current {
        Some(Opcode::Jump(offset)) => {
            vm.registers.code_ptr = offset;
            return Ok(());
        }
        Some(Opcode::JumpTrue(offset)) => {
            if let Value::Boolean(Boolean::True) = vm.pop()? {
                vm.registers.code_ptr = offset;
            } else {
                vm.registers.code_ptr += 1;
            }
            return Ok(());
        }
        Some(Opcode::JumpFalse(offset)) => {
            if let Value::Boolean(Boolean::False) = vm.pop()? {
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
            match &[left, right] {
                [Value::Literal(left_value), Value::Literal(right_value)] => {
                    vm.push(Value::Literal(left_value + right_value))?;
                }
                _ => {
                    return Err(ExecuteError::InvalidValue);
                }
            }
        }
        Some(Opcode::BinarySub) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            match &[left, right] {
                [Value::Literal(left_value), Value::Literal(right_value)] => {
                    vm.push(Value::Literal(left_value - right_value))?;
                }
                _ => {
                    return Err(ExecuteError::InvalidValue);
                }
            }
        }
        Some(Opcode::BinaryMul) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            match &[left, right] {
                [Value::Literal(left_value), Value::Literal(right_value)] => {
                    vm.push(Value::Literal(left_value * right_value))?;
                }
                _ => {
                    return Err(ExecuteError::InvalidValue);
                }
            }
        }
        Some(Opcode::BinaryDiv) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            match &[left, right] {
                [Value::Literal(left_value), Value::Literal(right_value)] => {
                    vm.push(Value::Literal(left_value / right_value))?;
                }
                _ => {
                    return Err(ExecuteError::InvalidValue);
                }
            }
        }
        Some(Opcode::Pop) => {
            vm.pop()?;
        }
        Some(Opcode::Push(value)) => {
            vm.push(value)?;
        }
        Some(Opcode::Compare) => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            let result = if left == right {
                Boolean::True
            } else {
                Boolean::False
            };
            vm.push(Value::Boolean(result))?;
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
