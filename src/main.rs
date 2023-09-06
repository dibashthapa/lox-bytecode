use chunk::{Chunk, OpCode};

mod chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test chunk");
    chunk.free();
}
