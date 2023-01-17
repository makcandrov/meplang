use std::collections::HashMap;

use crate::parser::error::new_error_from_ast;
use crate::{parser::parser::Ast, ast::file::MeplangFile};
use crate::parser::parser::Rule;

pub fn pre_process(
    code: &str,
    file: MeplangFile,
    contract_name: String,
) -> Result<(), pest::error::Error<Rule>>{
    let mut contract_names = HashMap::<String, usize>::new();
    for i in 0..file.0.len() {
        let name = &file.0[i].name.0;
        if contract_names.insert(name.clone(), i).is_some() {
            return Err(
                new_error_from_ast(code, &file.0[i].name, &format!("Name {} already used", name))
            )
        }
    }

    Ok(())
}
