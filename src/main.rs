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
    let constant = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);
    chunk.write_opcode(OpCode::OpReturn, 123);
    chunk.disassemble("test chunk");
    vm.interpret(&chunk);
    vm.free();
    chunk.free();
}
