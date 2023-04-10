#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Literal(i64),
    Boolean(bool),
    Reference(usize)
}

#[derive(Clone, Debug)]
pub enum Opcode {
    Noop,

    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,

    Push(Value),
    PushLocal(usize),
    Pop,

    Jump,
    JumpTrue,
    JumpFalse,
    Call(usize),
    Return(usize),
    Compare
}
