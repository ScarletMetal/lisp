mod bytecode;
mod vm;

use bytecode::{Opcode, Register, Value};

fn main() {
    let code = vec![
        Opcode::Push(Value::Literal(5)),
        Opcode::Pop(Register::Arithmetic(0)),
    ];
    let mut vm = vm::frame::VmFrame::new(code.clone());

    while vm.registers.code_ptr < code.len() {
        vm::frame::execute(&mut vm).expect("Woops!");
    }

    let cpy = vm.registers.stack_ptr.clone();

    println!(
        "value = {:?} mem = {:?} stack={:?}",
        vm.registers.r,
        vm.stack,
        cpy
    )
}
