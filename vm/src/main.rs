mod bytecode;
mod vm;

use bytecode::{Opcode, Value};

fn main() {
    let code = vec![
        Opcode::Push(Value::Literal(5)),
        Opcode::Push(Value::Literal(4)),
        Opcode::BinaryAdd,
        Opcode::Push(Value::Literal(10)),
        Opcode::BinaryMul,
        Opcode::Pop,
        Opcode::Push(Value::Literal(4)),
        Opcode::Push(Value::Literal(4)),
        Opcode::Compare,
        Opcode::Push(Value::Literal(322))
    ];
    let mut vm = vm::vm::Vm::new(&code);

    while vm.code_ptr < code.len() {
        vm::execute::execute(&mut vm);
        println!("stack={:?} stack_ptr={:?}", vm.stack, vm.stack_ptr)
    }
}
