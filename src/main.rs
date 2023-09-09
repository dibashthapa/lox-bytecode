use chunk::Chunk;
use opcode::OpCode;
use vm::Vm;

mod chunk;
mod value;
mod opcode;
mod vm;
fn main() {
    let mut vm = Vm::new();

    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.0);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(constant, 123);


    let constant = chunk.add_constant(2.0);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_opcode(OpCode::Multiply, 123);

    let constant= chunk.add_constant(3.0);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_opcode(OpCode::Add, 123);

    chunk.write_opcode(OpCode::Return, 123);


    vm.interpret(&chunk);
    chunk.free();
    vm.free();
}
