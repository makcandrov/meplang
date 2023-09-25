use super::artifacts::Artifacts;
use super::compile::compile_contracts;
use super::settings::CompilerSettings;
use crate::ast::RFile;
use crate::pre_processing::pre_processing::pre_process;

pub fn compile_file(path: &str, contract_name: &str, settings: CompilerSettings) -> Result<Artifacts, String> {
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

    let pre_processed = match pre_process(&input, r_file, contract_name, &settings.variables) {
        Ok(pre_processed) => pre_processed,
        Err(err) => {
            return Err(format!("Pre-processing failed:\n{}", err));
        },
    };

    let artifacts = compile_contracts(pre_processed, settings);

    Ok(artifacts)
}
