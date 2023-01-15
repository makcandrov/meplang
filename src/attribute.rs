
#[derive(Debug, Clone)]
pub enum Attribute {
    AssumeAttribute(AssumeAttribute),
    GlobalAssumeAttribute(AssumeAttribute),
}

#[derive(Debug, Clone)]

pub struct AssumeAttribute {
    pub opcode: AssumableOpcode,
    pub value: usize,
}

#[derive(Debug, Clone)]
pub enum AssumableOpcode {
    ChainId,
    CalldataSize,
    Msize,
}