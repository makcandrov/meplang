use bytes::Bytes;

use crate::{ast::RFile, pre_processing::pre_processing::pre_process};

use super::compile::compile_contracts;

pub fn compile_file(path: &str, contract_name: &str) -> Result<Bytes, String> {
    let input = match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => return Err(format!("Could not open file `{}`: {}", path, err.to_string())),
    };

    let r_file = match RFile::new(input.clone()) {
        Ok(r_file) => r_file,
        Err(err) => {
            return Err(format!("Parsing failed:\n{}", err));
        },
    };

    // dbg!(&meplang_file);

    let pre_processed = match pre_process(&input, r_file, contract_name) {
        Ok(pre_processed) => pre_processed,
        Err(err) => {
            return Err(format!("Pre-processing failed:\n{}", err));
        },
    };

    // dbg!(&pre_processed);

    let compiled = compile_contracts(pre_processed);

    Ok(compiled)
}
