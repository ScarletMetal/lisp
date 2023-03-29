#[derive(Clone, Copy, Debug)]
pub enum Register {
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
    JumpTrue(usize),
    JumpFalse(usize),
    Compare
}
