use std::collections::{HashMap, HashSet};

use bytes::Bytes;

use crate::ast::attribute::WithAttributes;
use crate::ast::contract::RContract;
use crate::parser::error::{new_error_from_located, new_generic_error};
use crate::{parser::parser::Located, ast::file::RFile};
use crate::parser::parser::Rule;

pub struct Contract {
    pub main: bool,
    pub name: String,
    pub contract_dependencies: Vec<usize>,
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
    let mut main_index: Option<usize> = None;
    let mut contract_names = HashMap::<String, usize>::new();

    for r_contract_with_attr in &file.0 {
        let r_contract = &r_contract_with_attr.inner.inner;
        let name = r_contract.name();
        if contract_names.insert(name.to_owned(), contract_names.len()).is_some() {
            return Err(new_error_from_located(
                code,
                &r_contract.name,
                &format!("Name `{}` already used", name)
            ));
        }
        if name == &contract_name {
            main_index = Some(contract_names.len() - 1);
        }
    }

    let Some(main_index) = main_index else {
        return Err(new_generic_error(
            format!("Contract `{}` not found", contract_name)
        ));
    };

    let mut pre_processed_contracts = HashMap::<usize, Contract>::new();
    let mut pre_processing_queue = vec![main_index];

    while let Some(index_to_process) = pre_processing_queue.pop() {
        let contract = pre_process_contract(
            code,
            &file.0[index_to_process],
            index_to_process == main_index,
            &contract_names,
        )?;

        for dependency in &contract.contract_dependencies {
            if 
                pre_processed_contracts.get(dependency).is_some() ||
                pre_processing_queue.contains(dependency) 
            {
                return Err(new_generic_error(
                    format!("Recursive contract dependencies unhandled")
                ));
            }
            pre_processing_queue.push(*dependency);
        }

        pre_processed_contracts.insert(index_to_process, contract);
    }

    for index in 0..file.0.len() {
        if pre_processed_contracts.get(&index).is_none() {
            log::warn!("{}", new_error_from_located(
                code,
                file.0[index].inner_located(),
                &format!("Unused contract")
            ));
        }
    }

    Ok(())
}

pub fn pre_process_contract(
    code: &str,
    r_contract_with_attr: &Located<WithAttributes<RContract>>,
    main: bool,
    contract_names: &HashMap<String, usize>,
) -> Result<Contract, pest::error::Error<Rule>> {
    let mut constants = HashMap::<String, Bytes>::new();
    let mut contract_dependencies = HashSet::<usize>::new();

    let r_contract = &r_contract_with_attr.inner.inner;
    for constant in &r_contract.constants {
        let value = constant.value.inner.clone();
        if value.len() >= 32 {
            return Err(new_error_from_located(
                code,
                &constant.value,
                &format!("Constants cannot exceed 32 bytes")
            ));
        }
        if constants.insert(constant.name.0.clone(), constant.value.inner.clone()).is_some() {
            return Err(new_error_from_located(
                code,
                &constant.name,
                &format!("Name {} already used", constant.name.0)
            ));
        }
    }

    let mut blocks = Vec::<Block>::new();
    let mut main_index: Option<usize> = None;
    let mut block_names = HashMap::<String, usize>::new();

    // for r_block in &r_contract.blocks {
    //     let name = r_block.name();
    //     if contract_names.insert(name.to_owned(), contract_names.len()).is_some() {
    //         return Err(new_error_from_ast(
    //             code,
    //             &r_contract.name,
    //             &format!("Name `{}` already used", name)
    //         ));
    //     }
    //     if name == &contract_name {
    //         main_index = Some(contract_names.len() - 1);
    //     }
    // }

    // let Some(main_index) = main_index else {
    //     return Err(new_generic_error(
    //         format!("Contract `{}` not found", contract_name)
    //     ));
    // };

    Ok(Contract {
        main,
        name: r_contract.name().to_owned(),
        contract_dependencies: vec![],
        blocks,
    })
}
