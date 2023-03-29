#[derive(Clone, Copy, Debug)]
pub enum Register {
    Flags,
    Stack,
    Code,
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Literal(i64),
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Noop,

    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,

    Push(Value),
    Pop(Register),

    Jump(usize),
    JumpZero(usize),
    JumpNotZero(usize),
    Compare(Value, Value)
}
