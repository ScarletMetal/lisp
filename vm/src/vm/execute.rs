use crate::bytecode::{Opcode, Value};
use crate::vm::vm::Vm;

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
    NotCallable(Value),
}

fn execute_opcode(vm: &mut Vm, opcode: &Opcode) -> ExecuteResult<()> {
    match opcode {
        Opcode::Jump => {
            return vm.jump();
        }
        Opcode::JumpTrue => {
            if let Value::Boolean(true) = vm.pop()? {
                return vm.jump();
            } else {
                vm.code_ptr += 1;
            }
            return Ok(());
        }
        Opcode::JumpFalse => {
            if let Value::Boolean(false) = vm.pop()? {
                return vm.jump();
            } else {
                vm.code_ptr += 1;
            }
            return Ok(());
        }
        _ => {}
    }

    match opcode {
        Opcode::BinaryAdd => {
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
        Opcode::BinarySub => {
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
        Opcode::BinaryMul => {
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
        Opcode::BinaryDiv => {
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
        Opcode::Pop => {
            vm.pop()?;
        }
        Opcode::Push(value) => {
            vm.push(value.clone())?;
        }
        Opcode::Compare => {
            let left = vm.pop()?;
            let right = vm.pop()?;
            let result = if left == right { true } else { false };
            vm.push(Value::Boolean(result))?;
        }
        Opcode::Noop => {}
        _ => {
            return Err(ExecuteError::UnhandledOpcode(opcode.clone()));
        }
    }

    vm.code_ptr += 1;
    Ok(())
}

pub fn execute(vm: &mut Vm) -> Result<(), ExecuteError> {
    let current = vm.code_chunk.get(vm.code_ptr);

    if let Some(opcode) = current {
        execute_opcode(vm, opcode)
    } else {
        Err(ExecuteError::NoOpcode)
    }
}
