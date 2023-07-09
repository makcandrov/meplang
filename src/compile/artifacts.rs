use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use bytes::Bytes;

use super::settings::{serialize_bytes, deserialize_bytes};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Artifacts {
    pub main: String,
    pub contracts: HashMap<String, ContractArtifacts>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContractArtifacts {
    pub blocks: HashMap<String, BlockArtifacts>,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub bytecode: Bytes,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BlockArtifacts {
    pub pc: usize,
    pub size: usize,
}

impl Artifacts {
    pub fn main_bytecode(&self) -> &Bytes {
        &self.contracts.get(&self.main).unwrap().bytecode
    }
}

impl ContractArtifacts {
    pub fn set_pc(&mut self, block_name: &str, pc: usize) {
        assert!(self
            .blocks
            .insert(block_name.to_owned(), BlockArtifacts { pc, size: 0 })
            .is_none());
    }

    pub fn set_size(&mut self, block_name: &str, end: usize) {
        let ba = self.blocks.get_mut(block_name).unwrap();
        assert!(end >= ba.pc);
        ba.size = end - ba.pc;
    }
}
