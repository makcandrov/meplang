use std::collections::{HashMap, HashSet};

use bytes::Bytes;

use crate::ast::*;
use crate::parser::error::{new_error_from_located, new_generic_error, new_error_from_location};
use crate::parser::parser::Located;
use crate::parser::parser::Rule;
use crate::pre_processing::attribute::Attributes;
use crate::pre_processing::dependencies::DependencyTree;
use crate::pre_processing::remapping::remap_blocks;

use super::attribute::Attribute;
use super::block_flow::{BlockFlow, analyze_block_flow, BlockFlowItem, BlockFlowPush, BlockFlowPushInner, BlockFlowBlockRef};
use super::queue::DedupQueue;
use super::remapping::remap_contracts;

#[derive(Clone, Default, Debug)]
pub struct Contract {
    pub blocks: Vec<Block>,
    pub last: bool,
}

#[derive(Clone, Default, Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Clone, Debug)]
pub enum BlockItem {
    Bytes(Bytes),
    Contract(usize),
    Push(Push),
}

#[derive(Clone, Debug)]
pub struct Push {
    pub attributes: Attributes,
    pub inner: PushInner,
}

#[derive(Clone, Debug)]
pub enum PushInner {
    Constant(Bytes),
    BlockSize { index: usize, start: usize, end: usize},
    BlockPc { index: usize, line: usize },
}

pub fn pre_process(
    input: &str,
    r_file: RFile,
    contract_name: &str,
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
        if name == contract_name {
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
        // log::info!("Pre-processing contract {}", &r_file.0[index_to_process].inner().name_str());
        contracts_dependency_tree.add_node_if_needed(&index_to_process);
        let (contract, dependencies) = pre_process_contract(
            input,
            &r_file.0[index_to_process],
            &contract_attributes[index_to_process],
            &contract_names,
        )?;

        for dependency in dependencies {
            contracts_queue.insert_if_needed(dependency);
            contracts_dependency_tree.insert_if_needed(&index_to_process, &dependency);
        }

        contracts.insert(index_to_process, contract);
    }

    for index in 0..r_file.0.len() {
        if contracts.get(&index).is_none() {
            log::warn!("{}", new_error_from_located(
                input,
                r_file.0[index].inner(),
                &format!("Unused contract `{}`", r_file.0[index].inner().name_str())
            ));
        }
    }

    let mut remapping_indexes = Vec::<usize>::new();
    while let Some(contract_index) = contracts_dependency_tree.pop_leaf() {
        remapping_indexes.push(contract_index);
    }

    if !contracts_dependency_tree.is_empty() {
        return Err(new_generic_error("Recursive contracts unhandled".to_owned()));
    }

    remapping_indexes.reverse();

    Ok(remap_contracts(contracts, &remapping_indexes))
}

pub fn pre_process_contract(
    input: &str,
    r_contract_with_attr: &Located<WithAttributes<Located<RContract>>>,
    default_attributes: &Attributes,
    contract_names: &HashMap<String, usize>,
) -> Result<(Contract, HashSet<usize>), pest::error::Error<Rule>> {
    let r_contract = &r_contract_with_attr.inner.inner;

    let constants = extract_constants(input, &r_contract.constants, contract_names)?;

    let mut block_attributes = vec![Vec::<Attribute>::new(); r_contract.blocks.len()];

    let mut main_index: Option<usize> = None;
    let mut last_index: Option<usize> = None;
    let mut block_names = HashMap::<String, usize>::new();

    let mut blocks_queue = DedupQueue::<usize>::new();

    for block_index in 0..r_contract.blocks.len() {
        let r_block_with_attr = &r_contract.blocks[block_index];
        for r_attribute in &r_block_with_attr.attributes {
            let attribute = Attribute::from_r_attribute(input, r_attribute)?;
            if !r_block_with_attr.inner().abstr {
                if attribute.is_block_attribute() {
                    if attribute.is_last() {
                        if last_index.replace(block_index).is_some() {
                            return Err(new_error_from_located(
                                input,
                                r_attribute,
                                "This contract has already a block marked with the attribute `last`.",
                            ));
                        }
                    } else if attribute.is_keep() {
                        blocks_queue.insert_if_needed(block_index);
                    } else if attribute.is_main() {
                        if main_index.replace(block_index).is_some() {
                            return Err(new_error_from_located(input, r_attribute, "A block is already marked as main."));
                        }
                    } else {
                        block_attributes[block_index].push(attribute);
                    }
                } else {
                    return Err(new_error_from_located(input, r_attribute, "Invalid block attribute."));
                }
            } else {
                if attribute.is_abstract_block_attribute() {
                    block_attributes[block_index].push(attribute);
                } else {
                    return Err(new_error_from_located(input, r_attribute, "Invalid abstract block attribute."));
                }
            }
        }
        
        let r_block = &r_block_with_attr.inner().inner;
        let block_name = r_block.name_str();

        if contract_names.contains_key(block_name)
            || constants.contains_key(block_name)
            || block_names.insert(block_name.to_owned(), block_names.len()).is_some()
        {
            return Err(new_error_from_located(
                input,
                &r_contract.name,
                &format!("Name `{}` already used", block_name),
            ));
        }
        if block_name == "main" && main_index.replace(block_index).is_some(){
            return Err(new_error_from_located(input, &r_block.name, "A block is already marked as main."));
        }
    }

    let main_index = main_index;
    let last_index = last_index;
    let block_attributes = block_attributes;
    let block_names = block_names;

    let Some(main_index) = main_index else {
        return Err(new_error_from_located(
            input,
            &r_contract,
            &format!("Block `main` not found in contract `{}`", r_contract.name_str())
        ));
    };
    blocks_queue.insert_if_needed(main_index);

    let mut blocks_flow = HashMap::<usize, BlockFlow>::new();
    let mut contract_dependencies = HashSet::<usize>::new();
    let mut block_dependency_tree = DependencyTree::<usize>::new();

    while let Some(index_to_process) = blocks_queue.pop() {
        block_dependency_tree.add_node_if_needed(&index_to_process);
        let block = analyze_block_flow(
            input,
            &r_contract.blocks[index_to_process],
            &constants,
            &contract_names,
            &block_names,
            &mut contract_dependencies,
        )?;

        for (dependency, strong) in &block.dependencies {
            blocks_queue.insert_if_needed(*dependency);
            if *strong {
                block_dependency_tree.insert_if_needed(dependency, &index_to_process);
            }
        }

        blocks_flow.insert(index_to_process, block);
    }
    let blocks_flow = blocks_flow;

    for block_index in 0..r_contract.blocks.len() {
        if blocks_flow.get(&block_index).is_none() {
            log::warn!("{}", new_error_from_located(
                input,
                &r_contract.blocks[block_index],
                &format!("Unused block `{}`", &r_contract.blocks[block_index].inner().name_str())
            ));
        }
    }

    let mut blocks_queue: Vec<usize> = block_dependency_tree.leaves().iter().map(|x| *x).collect();
    // println!("roots found {:?}", blocks_queue.iter().map(|x| r_contract.blocks[*x].inner().name_str()).collect::<Vec<&str>>());

    while block_dependency_tree.pop_leaf().is_some() {}

    if !block_dependency_tree.is_empty() {
        return Err(new_generic_error("Recursive blocks unhandled".to_owned()));
    }

    let mut blocks = HashMap::<usize, Block>::new();
    let mut unique_dereferences = HashSet::<usize>::new();
    let mut new_positions = HashMap::<usize, BlockPosition>::new();
    let mut remapping = Vec::<usize>::new();
    remapping.push(main_index);

    while let Some(index_to_process) = blocks_queue.pop() {
        if index_to_process != main_index && index_to_process != last_index.unwrap_or(main_index) {
            remapping.push(index_to_process);
        }

        let block = pre_process_block(
            input,
            index_to_process,
            &r_contract.blocks,
            &blocks_flow,
            BlockPreProcessingContext::new_root(index_to_process),
            &mut [index_to_process].into(),
            &mut default_attributes.clone(),
            &block_attributes,
            &mut unique_dereferences,
            &mut new_positions,
        )?;

        blocks.insert(index_to_process, block);
    }
    if let Some(last_index) = last_index {
        remapping.push(last_index);
    }

    Ok((Contract {
        blocks: remap_blocks(blocks, &remapping, &new_positions),
        last: last_index.is_some(),
    }, contract_dependencies))
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

#[derive(Clone, Debug)]
pub struct BlockPreProcessingContext {
    pub root_index: usize,
    pub inside_abstract: bool,
    pub line_index: usize,
}

#[derive(Clone, Debug)]
pub struct BlockPosition {
    pub root_index: usize,
    pub start: usize,
    pub end: usize,
}

impl BlockPreProcessingContext{
    pub fn new_root(index: usize) -> Self {
        Self {
            root_index: index,
            inside_abstract: false,
            line_index: 0,
        }
    }

    pub fn next_context(&self, inside_abstract: bool, line_index: usize) -> Self {
        Self {
            root_index: self.root_index,
            inside_abstract,
            line_index,
        }
    }
}

fn pre_process_block(
    input: &str,
    index_to_process: usize,
    r_blocks: &Vec<Located<WithAttributes<Located<RBlock>>>>,
    blocks_flow: &HashMap<usize, BlockFlow>,
    context: BlockPreProcessingContext,
    parents: &mut HashSet<usize>,
    current_attributes: &mut Attributes,
    block_attributes: &Vec<Vec<Attribute>>,
    unique_dereferences: &mut HashSet<usize>,
    new_positions: &mut HashMap<usize, BlockPosition>,
) -> Result<Block, pest::error::Error<Rule>> {
    // log::info!("Pre-processing block {}", &r_blocks[index_to_process].inner().name_str());

    current_attributes.apply_many(block_attributes[index_to_process].clone());

    let mut items = Vec::<BlockItem>::new();

    let block_flow = blocks_flow.get(&index_to_process).unwrap();

    for block_flow_item in &block_flow.items {
        match block_flow_item {
            BlockFlowItem::Bytes(bytes) => items.push(BlockItem::Bytes(bytes.clone())),
            BlockFlowItem::Contract(contract_index) => items.push(BlockItem::Contract(*contract_index)),
            BlockFlowItem::Push(BlockFlowPush { attributes, inner }) => {
                current_attributes.apply_many(attributes.clone());
                items.push(BlockItem::Push(Push {
                    attributes: current_attributes.clone(),
                    inner: match inner {
                        BlockFlowPushInner::Constant(bytes) => PushInner::Constant(bytes.clone()),
                        BlockFlowPushInner::BlockPc(index) => PushInner::BlockPc { index: *index, line: 0 },
                        BlockFlowPushInner::BlockSize(index) => PushInner::BlockSize { index: *index, start: 0, end: 0 },
                    }
                }));
            },
            BlockFlowItem::BlockEsp(BlockFlowBlockRef { index: block_index, location, attributes }) => {
                current_attributes.apply_many(attributes.clone());
                if !r_blocks[*block_index].inner().abstr {
                    return Err(new_error_from_location(input, &location, "Use the `*` to refer to a non abstract block."));
                }

                if parents.contains(block_index) {
                    return Err(new_error_from_location(input, &location, "Recursive block references unhandled"));
                }

                parents.insert(*block_index);
                let Block { items: mut sub_items } = pre_process_block(
                    input,
                    *block_index,
                    r_blocks,
                    blocks_flow,
                    context.next_context(true, items.len()),
                    parents,
                    current_attributes,
                    block_attributes,
                    unique_dereferences,
                    new_positions
                )?;
                parents.remove(&block_index);
                items.append(&mut sub_items);
                current_attributes.apply_many(blocks_flow.get(block_index).unwrap().end_attributes.clone());
            },
            BlockFlowItem::BlockStar(BlockFlowBlockRef { index: block_index, location, attributes }) => {
                current_attributes.apply_many(attributes.clone());
                if context.inside_abstract {
                    return Err(new_error_from_location(
                        input,
                        &location,
                        "Cannot refer to non-abstract block inside an abstract block."
                    ));
                }

                if r_blocks[*block_index].inner().abstr {
                    return Err(new_error_from_location(input, &location, "Use the `&` to refer to an abstract block."));
                }

                if unique_dereferences.contains(block_index) {
                    return Err(new_error_from_location(input, &location, "This non-abtrsact block has already been dereferenced once."));
                }
                // println!("dereferencing {} inside {} (root {})" ,
                //     r_blocks[*block_index].inner().name_str(),
                //     r_blocks[index_to_process].inner().name_str(),
                //     r_blocks[context.root_index].inner().name_str(),
                // );
                unique_dereferences.insert(*block_index);

                if parents.contains(block_index) {
                    return Err(new_error_from_location(input, &location, "Recursive block references unhandled"));
                }

                parents.insert(*block_index);
                let Block { items: mut sub_items } = pre_process_block(
                    input,
                    *block_index,
                    r_blocks,
                    blocks_flow,
                    context.next_context(false, items.len()),
                    parents,
                    current_attributes,
                    block_attributes,
                    unique_dereferences,
                    new_positions
                )?;
                parents.remove(&block_index);
                items.append(&mut sub_items);
                current_attributes.apply_many(blocks_flow.get(block_index).unwrap().end_attributes.clone());
            },
        }
    }

    new_positions.insert(index_to_process, BlockPosition {
        root_index: context.root_index,
        start: context.line_index,
        end: context.line_index + items.len(),
    });

    Ok(Block { items })
}
