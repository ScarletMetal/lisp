use crate::bytecode::{Opcode, Value};
use crate::vm::vm::{Vm, STACK_SIZE};

pub type ExecuteResult<T> = Result<T, ExecuteError>;

#[derive(Debug)]
pub enum ExecuteError {
    EmptyStack,
    InvalidRegister,
    NoOpcode,
    StackOverflow,
    UnhandledOpcode(Opcode),
    UnknownRegister,
    ZeroDivision,
    InvalidValue,
    ValueIsNotCallable,
}

pub fn execute(vm: &mut Vm) -> Result<(), ExecuteError> {
    let current = { vm.code.get(vm.code_ptr).map(Clone::clone) };

    match current {
        Some(Opcode::Jump) => {
            return vm.jump();
        }
        Some(Opcode::JumpTrue) => {
            if let Value::Boolean(true) = vm.pop()? {
                return vm.jump();
            } else {
                vm.code_ptr += 1;
            }
            return Ok(());
        }
        Some(Opcode::JumpFalse) => {
            if let Value::Boolean(false) = vm.pop()? {
                return vm.jump();
            } else {
                vm.code_ptr += 1;
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
            let result = if left == right { true } else { false };
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

    vm.code_ptr += 1;
    Ok(())
}
