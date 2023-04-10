use crate::bytecode::{Opcode, Value};
use crate::vm::vm::Vm;

pub type ExecuteResult<T> = Result<T, ExecuteError>;

#[derive(Debug)]
pub enum ExecuteError {
    EmptyCallStack,
    EmptyDataStack,
    InvalidReference,
    InvalidValue,
    LocalNotFound(usize),
    NoOpcode,
    UnhandledOpcode(Opcode),
    ZeroDivision,
}

fn execute_opcode(vm: &mut Vm, opcode: Opcode) -> ExecuteResult<()> {
    match opcode {
        Opcode::Jump => {
            let offset = vm.pop_ref()?;
            return vm.jump(offset);
        }
        Opcode::JumpTrue => {
            let offset = vm.pop_ref()?;
            if let Value::Boolean(true) = vm.pop()? {
                return vm.jump(offset);
            } else {
                vm.step()?;
            }
            return Ok(());
        }
        Opcode::JumpFalse => {
            let offset = vm.pop_ref()?;
            if let Value::Boolean(false) = vm.pop()? {
                return vm.jump(offset);
            } else {
                vm.step()?;
            }
            return Ok(());
        }
        Opcode::Call(num_params) => {
            let chunk_id = vm.pop_ref()?;
            vm.step()?;
            return vm.call(chunk_id, num_params);
        }
        Opcode::Return(num_params) => {
            return vm.ret(num_params);
        }
        _ => {}
    }

    match opcode {
        Opcode::BinaryAdd => {
            let right = vm.pop()?;
            let left = vm.pop()?;
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
            let right = vm.pop()?;
            let left = vm.pop()?;
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
            let right = vm.pop()?;
            let left = vm.pop()?;
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
            let right = vm.pop()?;
            let left = vm.pop()?;
            match &[left, right] {
                [_, Value::Literal(0)] => {
                    return Err(ExecuteError::ZeroDivision);
                }
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
            vm.push(value)?;
        }
        Opcode::PushLocal(index) => {
            let value = vm
                .get_locals()?
                .get(index)
                .ok_or(ExecuteError::LocalNotFound(index))?;
            vm.push(value.clone())?
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

    vm.step()?;
    Ok(())
}

pub fn execute(vm: &mut Vm) -> ExecuteResult<()> {
    let opcode = vm.get_current_opcode()?.clone();
    execute_opcode(vm, opcode)
}
