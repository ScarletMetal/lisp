#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Boolean {
    True,
    False
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    Literal(i64),
    Boolean(Boolean),
    Reference(i32)
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Noop,

    BinaryAdd,
    BinarySub,
    BinaryMul,
    BinaryDiv,

    Push(Value),
    Pop,

    Jump(usize),
    JumpTrue(usize),
    JumpFalse(usize),
    Compare
}
