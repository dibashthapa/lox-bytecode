use crate::{
    opcode::OpCode,
    value::{Value, ValueArray, print_value},
};

#[derive(Clone, Debug )]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_opcode(&mut self, code: OpCode, line: usize) {
        self.code.push(code.into());
        self.lines.push(line);
    }

    pub fn read(&self, ip:usize) -> u8 {
        self.code[ip]
    }  

    pub fn free(&mut self) {
        self.code = vec![];
        self.constants.free();
    }
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value) as u8
    }

    pub fn get_constant(&self, index:usize) -> Value {
        self.constants.values[index]
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{name:-16} {constant:4} ");
        print_value(self.constants.values[constant as usize]);
        println!();
        offset + 2
    }

    // pub fn disassemble(&mut self, name: &str) {
    //     println!("== {} ==", name);

    //     let mut offset = 0;

    //     while offset < self.code.len() {
    //         offset = self.disassemble_instruction(offset);
    //     }
    // }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        let instruction = self.code[offset].into();

        match instruction {
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract=> simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply=> simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide=> simple_instruction("OP_DIVIDE", offset),
        }
    }

}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}
