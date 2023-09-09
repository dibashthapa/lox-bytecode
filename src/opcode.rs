use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OPNegate = 3
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpConstant => write!(f, "OP_CONSTANT"),
            OpCode::OPNegate=> write!(f, "OP_NEGATE"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OPNegate,
            _ => unimplemented!("not implemented yet"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}
