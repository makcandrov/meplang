use std::collections::{HashMap, HashSet};

use bytes::Bytes;

use crate::ast::contract::RContract;
use crate::parser::error::new_error_from_ast;
use crate::{parser::parser::Located, ast::file::RFile};
use crate::parser::parser::Rule;

pub struct Contract {
    pub main: bool,
    pub name: String,
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub main: bool,
    pub name: String,

}

pub fn pre_process(
    code: &str,
    file: RFile,
    contract_name: String,
) -> Result<(), pest::error::Error<Rule>> {
    let mut contract_names = HashMap::<String, usize>::new();
    for i in 0..file.0.len() {
        let name = &file.0[i].name.0;
        if contract_names.insert(name.clone(), i).is_some() {
            return Err(new_error_from_ast(
                code,
                &file.0[i].name,
                &format!("Name {} already used", name)
            ));
        }
    }

    Ok(())
}

pub fn pre_process_contract(
    code: &str,
    contract_token: &Located<RContract>,
    contract_names: &HashMap<String, usize>,
) -> Result<(), pest::error::Error<Rule>> {
    let mut constants = HashMap::<String, Bytes>::new();
    let mut contract_dependencies = HashSet::<usize>::new();

    for constant in &contract_token.constants {
        if constants.insert(constant.name.0.clone(), constant.value.inner.clone()).is_some() {
            return Err(new_error_from_ast(
                code,
                &constant.name,
                &format!("Name {} already used", constant.name.0)
            ));
        }
    }
    Ok(())
}
