use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Return => write!(f, "OP_RETURN"),
            OpCode::Constant => write!(f, "OP_CONSTANT"),
            OpCode::Negate=> write!(f, "OP_NEGATE"),
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Subtract=> write!(f, "OP_SUBTRACT"),
            OpCode::Multiply=> write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Subtract,
            5 => OpCode::Multiply,
            6 => OpCode::Divide,
            _ => unimplemented!("not implemented yet"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}
