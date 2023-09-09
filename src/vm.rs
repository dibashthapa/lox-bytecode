use crate::{
    chunk::Chunk,
    opcode::OpCode,
    value::{Value},
};

#[derive(Clone)]
pub struct Vm {
    // instruction pointer
    pub ip: usize,
    stack: Vec<Value>,
    tos: Value,
}

#[derive(Clone, Debug)]
pub enum InterpretResult {
    InterpretOk,
    // InterpretCompileError,
    // InterpretRuntimeError,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: vec![],
            tos: 0.0,
        }
    }

    pub fn reset_stack(&mut self) {
        self.tos = self.stack[0];
    }
    pub fn free(&mut self) {}

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        return self.run(chunk);
    }

    pub fn push(&mut self, value: Value) {
        self.tos = value;
        self.tos += 1.0;
    }

    pub fn pop(&mut self) -> Value {
        self.tos -= 1.0;
        return self.tos;
    }

    pub fn debug_trace(&self) {}

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let opcode: OpCode = chunk.read(self.ip).into();
        self.ip += 1;
        opcode
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in self.stack.iter() {
                    print!("[  {slot}  ]");
                }
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte(chunk);

            match instruction {
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::OPNegate => {
                    let value = self.pop();
                    self.push(-value);
                }
            }
        }
    }

    pub fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = chunk.read(self.ip) as usize;
        self.ip += 1;
        chunk.get_constant(index)
    }
}
