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
