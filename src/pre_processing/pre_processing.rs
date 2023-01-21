use std::collections::{HashMap, HashSet};

use bytes::{BufMut, Bytes, BytesMut};

use crate::ast::*;
use crate::parser::error::{new_error_from_located, new_generic_error};
use crate::parser::parser::Located;
use crate::parser::parser::Rule;
use crate::pre_processing::attribute::Attributes;
use crate::pre_processing::dependencies::DependencyTree;

use super::attribute::Attribute;
use super::opcode::str_to_op;
use super::queue::DedupQueue;

#[derive(Clone, Default, Debug)]
pub struct Contract {
    pub dependencies: HashSet<usize>,
    pub blocks: Vec<Block>,
    pub main: usize,
    pub last: Option<usize>,
}

#[derive(Clone, Default, Debug)]
pub struct Block {
    pub items: Vec<WithAttributes<BlockItem>>,
    pub dependencies: HashSet<usize>,
}

#[derive(Clone, Debug)]
pub enum BlockItem {
    Bytes(Bytes),
    Contract(usize),
    Block(usize),
    Push(Push),
}

#[derive(Clone, Debug)]
pub enum Push {
    Constant(Bytes),
    ContractPc(usize),
    ContractSize(usize),
    BlockSize(usize),
    BlockPc(usize),
}

pub fn pre_process(
    input: &str,
    r_file: RFile,
    contract_name: String,
) -> Result<Vec<Contract>, pest::error::Error<Rule>> {
    let mut main_index: Option<usize> = None;
    let mut contract_names = HashMap::<String, usize>::new();
    let mut contract_attributes = vec![Attributes::default(); r_file.0.len()];

    for contract_index in 0..r_file.0.len() {
        let r_contract_with_attr = &r_file.0[contract_index];
        for r_attribute in &r_contract_with_attr.attributes {
            let attribute = Attribute::from_r_attribute(input, r_attribute)?;
            if attribute.is_contract_attribute() {
                contract_attributes[contract_index].apply(attribute);
            } else {
                return Err(new_error_from_located(input, r_attribute, "Invalid contract attribute"));
            }
        }

        let r_contract = &r_contract_with_attr.inner.inner;
        let name = r_contract.name_str();
        if contract_names.insert(name.to_owned(), contract_names.len()).is_some() {
            return Err(new_error_from_located(
                input,
                &r_contract.name,
                &format!("Name `{}` already used", name),
            ));
        }
        if name == &contract_name {
            // cannot happen twice
            main_index.replace(contract_index);
        }
    }

    let Some(main_index) = main_index else {
        return Err(new_generic_error(
            format!("Contract `{}` not found", contract_name)
        ));
    };

    let mut contracts = HashMap::<usize, Contract>::new();
    let mut contracts_queue = DedupQueue::<usize>::new();
    contracts_queue.insert_if_needed(main_index);

    let mut contracts_dependency_tree = DependencyTree::<usize>::new();

    while let Some(index_to_process) = contracts_queue.pop() {
        log::info!("Pre-processing contract {}", &r_file.0[index_to_process].inner().name_str());
        let contract = pre_process_contract(
            input,
            &r_file.0[index_to_process],
            &contract_attributes[index_to_process],
            &contract_names,
        )?;

        for dependency in &contract.dependencies {
            contracts_queue.insert_if_needed(*dependency);
            contracts_dependency_tree.insert_if_needed(&index_to_process, dependency);
        }

        contracts.insert(index_to_process, contract);
    }

    // for index in 0..file.0.len() {
    //     if contract_remapping_queue.remapping(&index).is_none() {
    //         log::warn!("{}", new_error_from_located(
    //             code,
    //             file.0[index].inner_located(),
    //             &format!("Unused contract `{}`", file.0[index].inner().name())
    //         ));
    //     }
    // }

    Ok(contracts.into_iter().map(|(_, c)| c).collect())
}

pub fn pre_process_contract(
    input: &str,
    r_contract_with_attr: &Located<WithAttributes<Located<RContract>>>,
    default_attributes: &Attributes,
    contract_names: &HashMap<String, usize>,
) -> Result<Contract, pest::error::Error<Rule>> {
    let r_contract = &r_contract_with_attr.inner.inner;

    let constants = extract_constants(input, &r_contract.constants, contract_names)?;

    let mut block_attributes = vec![default_attributes.clone(); r_contract.blocks.len()];

    let mut main_index: Option<usize> = None;
    let mut last_index: Option<usize> = None;
    let mut block_names = HashMap::<String, usize>::new();

    let mut blocks_dependency_tree = DependencyTree::<usize>::new();
    let mut block_types = vec![BlockType::Unused; r_contract.blocks.len()];

    let mut blocks_queue = DedupQueue::<usize>::new();

    for block_index in 0..r_contract.blocks.len() {
        let r_block_with_attr = &r_contract.blocks[block_index];
        for r_attribute in &r_block_with_attr.attributes {
            let attribute = Attribute::from_r_attribute(input, r_attribute)?;
            if attribute.is_block_attribute() {
                if attribute.is_last() {
                    if last_index.replace(block_index).is_some() {
                        return Err(new_error_from_located(
                            input,
                            r_attribute,
                            "This contract has already a block marked with the attribute `last`.",
                        ));
                    }
                }

                if attribute.is_keep() {
                    blocks_queue.insert_if_needed(block_index);
                }
                block_attributes[block_index].apply(attribute);
            } else {
                return Err(new_error_from_located(input, r_attribute, "Invalid block attribute."));
            }
        }

        let name = r_block_with_attr.inner().inner.name_str();

        if contract_names.contains_key(name)
            || constants.contains_key(name)
            || block_names.insert(name.to_owned(), block_names.len()).is_some()
        {
            return Err(new_error_from_located(
                input,
                &r_contract.name,
                &format!("Name `{}` already used", name),
            ));
        }
        if name == "main" {
            main_index = Some(block_index);
            block_types[block_index] = BlockType::Star;
            blocks_queue.insert_if_needed(block_index);
        }
    }

    let block_names = block_names;

    let Some(main_index) = main_index else {
        return Err(new_error_from_located(
            input,
            &r_contract,
            &format!("Block `main` not found in contract `{}`", r_contract.name_str())
        ));
    };

    let mut blocks = HashMap::<usize, Block>::new();
    let mut contract_dependencies = HashSet::<usize>::new();

    while let Some(index_to_process) = blocks_queue.pop() {
        log::info!("Pre-processing block {}", &r_contract.blocks[index_to_process].inner().name_str());
        let block = pre_process_block(
            input,
            &r_contract.blocks[index_to_process],
            &constants,
            &mut contract_dependencies,
            contract_names,
            &block_names,
            &mut block_types,
        )?;

        for dependency in &block.dependencies {
            blocks_queue.insert_if_needed(*dependency);
            blocks_dependency_tree.insert_if_needed(&index_to_process, dependency);
        }

        blocks.insert(index_to_process, block);
    }

    // for index in 0..r_contract.blocks.len() {
    //     if blocks_queue.remapping(&index).is_none() {
    //         log::warn!("{}", new_error_from_located(
    //             code,
    //             r_contract.blocks[index].inner_located(),
    //             &format!("Unused contract `{}`", r_contract.blocks[index].inner().name_str())
    //         ));
    //     }
    // }

    Ok(Contract {
        blocks: blocks.into_iter().map(|(_, c)| c).collect(),
        dependencies: contract_dependencies,
        main: main_index,
        last: last_index,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Unused,
    Keep,
    Main,
    Star,
    Esp,
}

pub fn extract_constants(
    input: &str,
    r_constants: &Vec<Located<RConstant>>,
    contract_names: &HashMap<String, usize>,
) -> Result<HashMap<String, Bytes>, pest::error::Error<Rule>> {
    let mut constants = HashMap::<String, Bytes>::new();

    for r_constant in r_constants {
        let constant_name = r_constant.name_str();
        let value = r_constant.value.inner.clone().0;
        if value.len() >= 32 {
            return Err(new_error_from_located(
                input,
                &r_constant.value,
                &format!("Constants cannot exceed 32 bytes"),
            ));
        }
        if contract_names.contains_key(constant_name)
            || constants.insert(constant_name.to_owned(), value.clone()).is_some()
        {
            return Err(new_error_from_located(
                input,
                &r_constant.name,
                &format!("Name {} already used", r_constant.name.0),
            ));
        }
    }
    Ok(constants)
}

pub fn pre_process_block(
    input: &str,
    r_block_with_attr: &Located<WithAttributes<Located<RBlock>>>,
    constants: &HashMap<String, Bytes>,
    contract_dependencies: &mut HashSet<usize>,
    contract_names: &HashMap<String, usize>,
    block_names: &HashMap<String, usize>,
    block_types: &mut Vec<BlockType>,
) -> Result<Block, pest::error::Error<Rule>> {
    let r_block = r_block_with_attr.inner();

    let mut items = Vec::<WithAttributes::<BlockItem>>::new();
    let mut block_dependencies = HashSet::<usize>::new();

    let mut current_attributes = Vec::<Located<RAttribute>>::new();
    let mut current_bytes: Option<BytesMut> = None;

    for item_with_attr in &r_block.items {
        for attr in &item_with_attr.attributes {
            current_attributes.push(attr.clone());
        }

        let item = item_with_attr.inner();

        if let RBlockItem::HexLitteral(hex_litteral) = &item.inner {
            append_or_create_bytes(&mut current_bytes, &hex_litteral.0);
            continue;
        }

        if let Some(c_bytes) = current_bytes.take() {
            items.push(WithAttributes::new_without_attribute(BlockItem::Bytes(c_bytes.into())));
        }
        match &item.inner {
            RBlockItem::HexLitteral(_) => unreachable!(),
            RBlockItem::BlockRef(RBlockRef::Star(variable)) => {
                let block_name = variable.as_str();
                let Some(block_index) = block_names.get(variable.as_str()) else {
                    return Err(new_error_from_located(
                        input,
                        item,
                        &format!("Block `{}` not found in this contract.", block_name)
                    ));
                };

                match block_types[*block_index] {
                    BlockType::Unused => block_types[*block_index] = BlockType::Star,
                    BlockType::Keep => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `*` operator on a block marked with `keep`."),
                        ));
                    },
                    BlockType::Main => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `*` operator on the main block."),
                        ));
                    },
                    BlockType::Star => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `*` operator two times on the same block."),
                        ));
                    },
                    BlockType::Esp => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `*` operator if the `&` operator has already been used."),
                        ));
                    },
                }

                items.push(WithAttributes::new(BlockItem::Block(*block_index), current_attributes));
                current_attributes = Vec::new();
                block_dependencies.insert(*block_index);
            }
            RBlockItem::BlockRef(RBlockRef::Esp(RVariableOrVariableWithField::Variable(variable))) => {
                let block_name = variable.as_str();
                let Some(block_index) = block_names.get(variable.as_str()) else {
                    return Err(new_error_from_located(
                        input,
                        item,
                        &format!("Block `{}` not found in this contract.", block_name)
                    ));
                };

                match block_types[*block_index] {
                    BlockType::Unused => block_types[*block_index] = BlockType::Esp,
                    BlockType::Keep => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `&` operator on a block marked with `keep`."),
                        ));
                    },
                    BlockType::Main => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `&` operator on the main block."),
                        ));
                    },
                    BlockType::Star => {
                        return Err(new_error_from_located(
                            input,
                            item,
                            &format!("Cannot use the `&` operator if the `*` operator has already been used."),
                        ));
                    },
                    BlockType::Esp => (),
                }

                items.push(WithAttributes::new(BlockItem::Block(*block_index), current_attributes));
                current_attributes = Vec::new();
                block_dependencies.insert(*block_index);
            }
            RBlockItem::BlockRef(RBlockRef::Esp(RVariableOrVariableWithField::VariableWithField(
                variable_with_field,
            ))) => {
                let field_name = variable_with_field.field.as_str();
                if field_name != "code" {
                    return Err(new_error_from_located(
                        input,
                        &variable_with_field.field,
                        &format!("Unknown field {}.", field_name),
                    ));
                }

                let variable_name = variable_with_field.variable.as_str();

                let Some(contract_index) = contract_names.get(variable_name) else {
                    return Err(new_error_from_located(
                        input,
                        &variable_with_field.variable,
                        &format!("Contract `{}` not found.", variable_name),
                    ));
                };

                items.push(WithAttributes::new_without_attribute(BlockItem::Contract(*contract_index)));
                contract_dependencies.insert(*contract_index);
            }
            RBlockItem::Variable(variable) => {
                if let Some(op) = str_to_op(variable.as_str()) {
                    push_or_create_bytes(&mut current_bytes, op);
                } else {
                    return Err(new_error_from_located(
                        input,
                        &item,
                        &format!("Contract `{}` not found.", variable.as_str()),
                    ));
                }
            }
            RBlockItem::Function(function) => {
                let function_name = function.name.as_str();

                if function_name.to_lowercase().as_str() != "push" {
                    return Err(new_error_from_located(
                        input,
                        &function.name,
                        &format!("Unknown function `{}`.", function_name),
                    ));
                }

                let push = match &function.arg.inner {
                    RFunctionArg::HexLitteral(hex_litteral) => Push::Constant(hex_litteral.0.clone()),
                    RFunctionArg::Variable(variable) => {
                        let Some(constant_value) = constants.get(variable.as_str()) else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                &format!("Invalid opcode `{}`.", variable.as_str()),
                            ));
                        };

                        Push::Constant(constant_value.clone())
                    }
                    RFunctionArg::VariableWithField(variable_with_field) => {
                        let field_name = variable_with_field.field.as_str();
                        let variable_name = variable_with_field.variable.as_str();
                        match field_name {
                            "pc" => {
                                if let Some(contract_index) = contract_names.get(variable_name) {
                                    contract_dependencies.insert(*contract_index);
                                    Push::ContractPc(*contract_index)
                                } else if let Some(block_index) = block_names.get(variable_name) {
                                    block_dependencies.insert(*block_index);
                                    Push::BlockPc(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Contract or block `{}` not found.", variable_name),
                                    ));
                                }
                            }
                            "size" => {
                                if let Some(contract_index) = contract_names.get(variable_name) {
                                    contract_dependencies.insert(*contract_index);
                                    Push::ContractSize(*contract_index)
                                } else if let Some(block_index) = block_names.get(variable_name) {
                                    block_dependencies.insert(*block_index);
                                    Push::BlockSize(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Contract or block `{}` not found.", variable_name),
                                    ));
                                }
                            }
                            _ => {
                                return Err(new_error_from_located(
                                    input,
                                    &variable_with_field.field,
                                    &format!("Unknown field `{}`.", field_name),
                                ))
                            }
                        }
                    }
                };

                items.push(WithAttributes::new(BlockItem::Push(push), current_attributes));
                current_attributes = Vec::new();
            }
        }
    }

    if let Some(c_bytes) = current_bytes.take() {
        items.push(WithAttributes::new(BlockItem::Bytes(c_bytes.into()), current_attributes));
        current_attributes = Vec::new();
    }
    if current_attributes.len() > 0 {
        items.push(WithAttributes::new(BlockItem::Bytes(Bytes::new()), current_attributes));
    }

    Ok(Block {
        items,
        dependencies: block_dependencies,
    })
}

fn append_or_create_bytes(current_bytes: &mut Option<BytesMut>, new_bytes: &Bytes) {
    if let Some(c_bytes) = current_bytes.as_mut() {
        c_bytes.extend_from_slice(new_bytes);
    } else {
        current_bytes.replace(new_bytes[..].into());
    }
}

fn push_or_create_bytes(current_bytes: &mut Option<BytesMut>, new_byte: u8) {
    if let Some(c_bytes) = current_bytes.as_mut() {
        c_bytes.put_u8(new_byte);
    } else {
        let mut c_bytes = BytesMut::new();
        c_bytes.put_u8(new_byte);
        current_bytes.replace(c_bytes);
    }
}
