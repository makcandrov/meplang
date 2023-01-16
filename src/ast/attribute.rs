use std::str::FromStr;

use crate::opcode::OpCode;


#[derive(Debug, Clone)]
pub enum Attribute {
    Assume(AssumeAttribute),
}

#[derive(Debug, Clone)]
pub struct AssumeAttribute {
    pub opcode: AssumableOpCode,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub enum AssumableOpCode {
    CHAINID,
    CALLDATASIZE,
    RETURNDATASIZE,
    MSIZE,
}

pub struct InvalidOpCodeError();

impl FromStr for AssumableOpCode {
    type Err = InvalidOpCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match OpCode::from_str(s) {
            Ok(OpCode::CHAINID) => Ok(Self::CHAINID),
            _ => Err(InvalidOpCodeError())
        }
    }
}