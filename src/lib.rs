#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

mod ast;
mod compile;
mod parser;
mod pre_processing;
mod types;

pub use compile::artifacts::{Artifacts, BlockArtifacts, ContractArtifacts};
pub use compile::file::compile_file;
pub use compile::settings::{CompilerSettings, FillingPatern};
