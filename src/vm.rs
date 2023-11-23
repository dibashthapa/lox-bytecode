use crate::{
    chunk::Chunk,
    opcode::OpCode,
    value::Value, compiler::compile,
};

#[derive(Clone)]
pub struct Vm {
    // instruction pointer
    pub ip: usize,
    stack: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        compile(source);
        InterpretResult::InterpretOk
    }

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
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return InterpretResult::InterpretOk;
                }
                OpCode::Constant => {
                    let constant = self.read_constant(chunk);
                    self.stack.push(constant);
                }
                OpCode::Negate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::Add => self.binary_op(|a,b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b|  a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b)
            }
        }
    }

    pub fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = chunk.read(self.ip) as usize;
        self.ip += 1;
        chunk.get_constant(index)
    }


    fn binary_op(&mut self, op: fn(a: Value , b: Value) -> Value ){
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }
}
