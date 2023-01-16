use primitive_types::H256;
use crate::{ast::attribute::Attribute, parser::parser::FromPair};
use crate::parser::parser::Rule;

#[derive(Debug, Clone, Default)]
pub struct Block {
    main: bool,
    id: usize,
    attributes: Vec<Attribute>,
    lines: Vec<BlockLine>,
}

#[derive(Debug, Clone)]

pub enum BlockLine {
    FlatOpcode(u8),
    PushConst(H256),
    PushVar([u8; 32]),
    Bytes(Box<[u8]>),
}

impl FromPair for Block {
    fn from_pair(block_decl_with_attr: pest::iterators::Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_decl_with_attr.as_rule() == Rule::block_decl_with_attr);

        Ok(Self::default())
    }
}