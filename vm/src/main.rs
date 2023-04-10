mod bytecode;
mod vm;

use bytecode::{Opcode, Value};
use vm::execute::ExecuteError;

fn main() {
    let first_chunk = vec![
        Opcode::Push(Value::Literal(500)),
        Opcode::Push(Value::Reference(2)),
        Opcode::Call(1),
    ];

    let second_chunk = Vec::from([
        Opcode::PushLocal(0),
        Opcode::Push(Value::Literal(0)),
        Opcode::Compare,
        Opcode::Push(Value::Reference(7)),
        Opcode::JumpFalse,
        Opcode::Push(Value::Literal(1)),
        Opcode::Return(1),
        Opcode::PushLocal(1),
        Opcode::PushLocal(0),
        Opcode::Push(Value::Literal(1)),
        Opcode::BinarySub,
        Opcode::Push(Value::Reference(1)),
        Opcode::Call(2),
        Opcode::PushLocal(1),
        Opcode::BinaryMul,
        Opcode::Return(1),
    ]);

    let third_chunk = Vec::from([
        Opcode::PushLocal(0),
        Opcode::Push(Value::Literal(0)),
        Opcode::Compare,
        Opcode::Push(Value::Reference(14)),
        Opcode::JumpTrue,
        Opcode::Push(Value::Literal(100)),
        Opcode::Push(Value::Literal(5)),
        Opcode::Push(Value::Reference(1)),
        Opcode::Call(2),
        Opcode::PushLocal(0),
        Opcode::Push(Value::Literal(1)),
        Opcode::BinarySub,
        Opcode::Push(Value::Reference(2)),
        Opcode::Call(1),
        Opcode::Return(0),
    ]);

    let context = vm::code::CodeVector::new(vec![
        first_chunk.clone(),
        second_chunk.clone(),
        third_chunk.clone(),
    ]);

    let mut vm = vm::Vm::new(context);

    loop {
        match vm::execute::execute(&mut vm) {
            Ok(()) => {}
            Err(ExecuteError::NoOpcode) => {
                println!("call_stack = {:?}", vm.call_stack);
                break;
            }
            err => {
                err.expect("Wtf?");
            }
        }
    }
}
