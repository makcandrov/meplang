use primitive_types::H256;

use crate::attribute::Attribute;

#[derive(Debug, Clone, Default)]
pub struct Block {
    main: bool,
    id: usize,
    attributes: Vec<Attribute>,
    expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]

pub enum Expression {
    FlatOpcode(u8),
    PushConst(H256),
    PushVar([u8; 32]),
    Bytes(Box<[u8]>),
}
