#[derive(Clone, Copy, Debug)]
pub enum Register {
    Arithmetic(usize),
    Flags,
    Stack,
    Code,
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Register(Register),
    Literal(i64),
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Noop,
    Mov(usize, Value),
    Add(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),

    Push(Value),
    Pop(Register),

    Jump(usize),
    JumpZero(usize),
    JumpNotZero(usize),
    Compare(Value, Value)
}
