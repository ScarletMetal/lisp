mod bytecode;
mod vm;

use bytecode::{Opcode, Register, Value};

fn main() {
    let code = vec![
        Opcode::Push(Value::Literal(5)),
        Opcode::Push(Value::Literal(4)),
        Opcode::BinaryAdd,
        Opcode::Push(Value::Literal(10)),
        Opcode::BinaryMul,
    ];
    let mut vm = vm::frame::VmFrame::new(code.clone());

    while vm.registers.code_ptr < code.len() {
        vm::frame::execute(&mut vm).expect("Woops!");
        println!(
            "stack={:?}",
            vm.stack
        )
    }
}
