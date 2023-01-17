use std::str::FromStr;

use pest::iterators::Pair;
use crate::parser::parser::Rule;
use crate::{opcode::OpCode, parser::parser::FromPair};


#[derive(Debug, Clone)]
pub enum RAttribute {
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

impl FromPair for RAttribute {
    fn from_pair(pair: Pair<Rule>) -> Result<Self, pest::error::Error<crate::parser::parser::Rule>> {
        todo!()
    }
}