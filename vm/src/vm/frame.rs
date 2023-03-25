use crate::bytecode::{Opcode, Register, Value};
use crate::vm::vm::{ExecuteError, STACK_SIZE};

pub struct VmFlags {
    zero: u8,
}

pub struct VmRegisters {
    pub r: [i64; 32],
    pub flags: VmFlags,
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
                r: [0; 32],
                flags: VmFlags { zero: 0 },
            },
        }
    }
}

fn _set_register(register: &Register, value: i64, vm: &mut VmFrame) -> Result<(), ExecuteError> {
    match register {
        Register::Arithmetic(index) if *index >= vm.registers.r.len() => {
            return Err(ExecuteError::UnknownRegister);
        }
        Register::Arithmetic(index) => {
            vm.registers.r[*index] = value;
        }
        Register::Stack => {
            vm.registers.stack_ptr = value as usize;
        }
        Register::Code => {
            vm.registers.code_ptr = value as usize;
        }
        _ => {
            return Err(ExecuteError::InvalidRegister);
        }
    }
    Ok(())
}

fn _retrive_register(register: &Register, vm: &mut VmFrame) -> Result<i64, ExecuteError> {
    match register {
        Register::Arithmetic(index) if *index >= vm.registers.r.len() => {
            Err(ExecuteError::UnknownRegister)
        }
        Register::Arithmetic(index) => Ok(vm.registers.r[*index]),
        Register::Code => Ok(vm.registers.code_ptr as i64),
        Register::Stack => Ok(vm.registers.stack_ptr as i64),
        _ => Err(ExecuteError::InvalidRegister),
    }
}

fn _retrieve_value(value: &Value, vm: &mut VmFrame) -> Result<i64, ExecuteError> {
    match value {
        Value::Literal(num) => Ok(*num),
        Value::Register(register) => _retrive_register(register, vm),
    }
}

fn _pop(vm: &mut VmFrame) -> Result<i64, ExecuteError> {
    if vm.registers.stack_ptr == 0 {
        return Err(ExecuteError::EmptyStack);
    }

    let value = vm.stack[vm.registers.stack_ptr - 1];
    vm.registers.stack_ptr -= 1;
    return Ok(value);
}

fn _push(value: i64, vm: &mut VmFrame) -> Result<(), ExecuteError> {
    if vm.registers.stack_ptr >= STACK_SIZE {
        return Err(ExecuteError::StackOverflow);
    }

    if vm.stack.len() >= vm.registers.stack_ptr {
        vm.stack.push(value);
    } else {
        vm.stack[vm.registers.stack_ptr] = value;
    }

    vm.registers.stack_ptr += 1;
    Ok(())
}

pub fn execute(vm: &mut VmFrame) -> Result<(), ExecuteError> {
    let current = { vm.code.get(vm.registers.code_ptr).map(Clone::clone) };

    match current {
        Some(Opcode::Jump(offset)) => {
            vm.registers.code_ptr = offset;
            return Ok(());
        }
        Some(Opcode::JumpZero(offset)) => {
            if vm.registers.flags.zero == 1 {
                vm.registers.code_ptr = offset;
            } else {
                vm.registers.code_ptr += 1;
            }
            return Ok(());
        }
        Some(Opcode::JumpNotZero(offset)) => {
            if vm.registers.flags.zero == 0 {
                vm.registers.code_ptr = offset;
            } else {
                vm.registers.code_ptr += 1;
            }
            return Ok(());
        }
        _ => {}
    }

    match current {
        Some(Opcode::Mov(reg, value)) => {
            vm.registers.r[reg] = _retrieve_value(&value, vm)?;
        }
        Some(Opcode::Add(left, value)) => {
            let right = _retrieve_value(&value, vm)?;
            vm.registers.r[left] += right;
        }
        Some(Opcode::Sub(left, value)) => {
            let right = _retrieve_value(&value, vm)?;
            vm.registers.r[left] -= right;
        }
        Some(Opcode::Mul(left, value)) => {
            let right = _retrieve_value(&value, vm)?;
            vm.registers.r[left] *= right;
        }
        Some(Opcode::Div(left, value)) => {
            let right = _retrieve_value(&value, vm)?;
            if right == 0 {
                return Err(ExecuteError::ZeroDivision);
            }
            vm.registers.r[left] /= right;
        }
        Some(Opcode::Pop(register)) => {
            let value = _pop(vm)?;
            _set_register(&register, value, vm)?;
        }
        Some(Opcode::Push(value)) => {
            let value = _retrieve_value(&value, vm)?;
            _push(value, vm)?;
        }
        Some(Opcode::Compare(left_value, right_value)) => {
            let left = _retrieve_value(&left_value, vm)?;
            let right = _retrieve_value(&right_value, vm)?;

            if left == right {
                vm.registers.flags.zero = 1;
            }
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
