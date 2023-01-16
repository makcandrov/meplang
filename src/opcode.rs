use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    CHAINID,
    RETURNDATASIZE,
}

pub struct InvalidOpcodeError();

impl FromStr for OpCode {
    type Err = InvalidOpcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chainid" => Ok(Self::CHAINID),
            "returndatasize" => Ok(Self::RETURNDATASIZE),
            _ => Err(InvalidOpcodeError())
        }
    }
}
