use crate::bytecode::Opcode;

pub const STACK_SIZE: usize = 2048;

#[derive(Debug)]
pub enum ExecuteError {
    EmptyStack,
    InvalidRegister,
    NoOpcode,
    StackOverflow,
    UnhandledOpcode(Opcode),
    UnknownRegister,
    ZeroDivision,
}
