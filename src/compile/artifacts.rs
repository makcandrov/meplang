use std::collections::HashMap;

use bytes::Bytes;

#[derive(Debug, Default, Clone)]
pub struct Artifacts {
    pub main: String,
    pub contracts: HashMap<String, ContractArtifacts>,
}

#[derive(Debug, Default, Clone)]
pub struct ContractArtifacts {
    pub blocks: HashMap<String, BlockArtifacts>,
    pub bytecode: Bytes,
}

#[derive(Debug, Default, Clone)]
pub struct BlockArtifacts {
    pub start: usize,
    pub size: usize,
}

impl Artifacts {
    pub fn main_bytecode(&self) -> &Bytes {
        &self.contracts.get(&self.main).unwrap().bytecode
    }
}

impl ContractArtifacts {
    pub fn set_start(&mut self, block_name: &str, start: usize) {
        assert!(self
            .blocks
            .insert(block_name.to_owned(), BlockArtifacts { start, size: 0 })
            .is_none());
    }

    pub fn set_size(&mut self, block_name: &str, end: usize) {
        let ba = self.blocks.get_mut(block_name).unwrap();
        assert!(end >= ba.start);
        ba.size = end - ba.start;
    }
}
