use std::fmt::Display;

#[derive(Debug)]
pub enum OpCode {
    OpReturn = 0,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            _ => unimplemented!("not implemented yet"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte.into());
    }

    pub fn free(&mut self) {
        self.code = vec![];
    }

    pub fn disassemble(&mut self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&mut self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let instruction = self.code[offset].into();

        match instruction {
            OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
            _ => {
                eprintln!("Unknown Opcode {}", instruction);
                return offset + 1;
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}
