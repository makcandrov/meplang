use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bytes::{Bytes, BytesMut};

use crate::ast::attribute::WithAttributes;
use crate::ast::block::{RBlock};
use crate::ast::contract::RContract;
use crate::parser::error::{new_error_from_located, new_generic_error};
use crate::{parser::parser::Located, ast::file::RFile};
use crate::parser::parser::Rule;

#[derive(Clone, Default, Debug)]
pub struct Contract {
    pub blocks: Vec<Block>,
}

#[derive(Clone, Default, Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Clone, Debug)]
pub struct BlockItem {
    pub assumes: HashMap<Bytes, u8>,
    pub content: BlockItemContent,
}

#[derive(Clone, Debug)]
pub enum BlockItemContent {
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
    code: &str,
    file: RFile,
    contract_name: String,
) -> Result<Vec<Contract>, pest::error::Error<Rule>> {
    // let mut main_index: Option<usize> = None;
    // let mut contract_names = HashMap::<String, usize>::new();

    // for r_contract_with_attr in &file.0 {
    //     let r_contract = &r_contract_with_attr.inner.inner;
    //     let name = r_contract.name();
    //     if contract_names.insert(name.to_owned(), contract_names.len()).is_some() {
    //         return Err(new_error_from_located(
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

    let mut contracts = Vec::<Contract>::new();
    // let mut contract_remapping_queue = RemappingQueue::<usize>::default();
    // contract_remapping_queue.insert_if_needed(main_index);

    // while let Some(index_to_process) = contract_remapping_queue.pop() {
    //     contracts.push(
    //         pre_process_contract(
    //             code,
    //             &file.0[index_to_process],
    //             &mut contract_remapping_queue,
    //             &contract_names,
    //         )?
    //     );

        // for dependency in &contract.contract_dependencies {
        //     if 
        //         pre_processed_contracts.get(dependency).is_some() ||
        //         pre_processing_queue.contains(dependency) 
        //     {
        //         return Err(new_generic_error(
        //             format!("Recursive contract dependencies unhandled")
        //         ));
        //     }
        //     pre_processing_queue.push(*dependency);
        // }

        // pre_processed_contracts.insert(index_to_process, contract);
    // }

    // for index in 0..file.0.len() {
    //     if contract_remapping_queue.remapping(&index).is_none() {
    //         log::warn!("{}", new_error_from_located(
    //             code,
    //             file.0[index].inner_located(),
    //             &format!("Unused contract `{}`", file.0[index].inner().name())
    //         ));
    //     }
    // }

    Ok(contracts)
}

// pub fn pre_process_contract(
//     code: &str,
//     r_contract_with_attr: &Located<WithAttributes<RContract>>,
//     contract_remapping_queue: &mut RemappingQueue<usize>,
//     contract_names: &HashMap<String, usize>,
// ) -> Result<Contract, pest::error::Error<Rule>> {
//     let mut constants = HashMap::<String, Bytes>::new();

//     let r_contract = &r_contract_with_attr.inner.inner;
//     for constant in &r_contract.constants {
//         let value = constant.value.inner.clone();
//         if value.len() >= 32 {
//             return Err(new_error_from_located(
//                 code,
//                 &constant.value,
//                 &format!("Constants cannot exceed 32 bytes")
//             ));
//         }
//         if constants.insert(constant.name.0.clone(), constant.value.inner.clone()).is_some() {
//             return Err(new_error_from_located(
//                 code,
//                 &constant.name,
//                 &format!("Name {} already used", constant.name.0)
//             ));
//         }
//     }
//     let constants = constants;

//     let mut blocks = Vec::<Block>::new();
//     let mut main_index: Option<usize> = None;
//     let mut block_names = HashMap::<String, usize>::new();

//     for r_block_with_attr in &r_contract.blocks {
//         let r_block = r_block_with_attr.inner_located();
//         let name = r_block.inner.name_str();
//         if 
//             contract_names.contains_key(name)||
//             constants.contains_key(name) ||
//             block_names.insert(name.to_owned(), block_names.len()).is_some()
//         {
//             return Err(new_error_from_located(
//                 code,
//                 &r_contract.name,
//                 &format!("Name `{}` already used", name)
//             ));
//         }
//         if name == "main" {
//             main_index = Some(block_names.len() - 1);
//         }
//     }

//     let Some(main_index) = main_index else {
//         return Err(new_error_from_located(
//             code,
//             &r_contract,
//             &format!("Block `main` not found in contract `{}`", r_contract.name())
//         ));
//     };


//     let mut blocks = Vec::<Block>::new();

//     let mut blocks_remapping_queue = RemappingQueue::<usize>::default();
//     blocks_remapping_queue.insert_if_needed(main_index);

//     while let Some(index_to_process) = blocks_remapping_queue.pop() {
//         blocks.push(
//             pre_process_block(
//                 code,
//                 &r_contract.blocks[index_to_process],
//                 &mut blocks_remapping_queue,
//                 &constants,
//                 contract_names,
//                 contract_remapping_queue,
//             )?
//         );
//     }

//     for index in 0..r_contract.blocks.len() {
//         if blocks_remapping_queue.remapping(&index).is_none() {
//             log::warn!("{}", new_error_from_located(
//                 code,
//                 r_contract.blocks[index].inner_located(),
//                 &format!("Unused contract `{}`", r_contract.blocks[index].inner().name_str())
//             ));
//         }
//     }

//     Ok(Contract {
//         blocks,
//     })
// }

// pub fn pre_process_block(
//     code: &str,
//     r_block_with_attr: &Located<WithAttributes<RBlock>>,
//     blocks_remapping_queue: &mut RemappingQueue<usize>,
//     constants: &HashMap<String, Bytes>,
//     contract_names: &HashMap<String, usize>,
//     contract_remapping_queue: &mut RemappingQueue<usize>,
// ) -> Result<Block, pest::error::Error<Rule>> {
//     let r_block = r_block_with_attr.inner();

//     let mut items = Vec::<BlockItem>::new();
//     let mut current_bytes: Option<BytesMut> = None;
//     for line in &r_block.lines {
//         match &line.inner {
//             RBlockLine::Var(var) => {
//                 let name = var.name();
//                 if let Some(bytes) = constants.get(name) {
//                     if let Some(c_bytes) = current_bytes.as_mut() {
//                         c_bytes.extend_from_slice(bytes);
//                     } else {
//                         current_bytes = Some(bytes[..].into());
//                     }
//                 } else {
//                     if let Some(bytes) = current_bytes.take() {
//                         items.push(BlockItem {
//                             assumes: HashMap::new(),
//                             content: BlockItemContent::Bytes(bytes.into()),
//                         })
//                     }
//                     if let Some(contract_old_index) = contract_names.get(name) {
//                         contract_remapping_queue.insert_if_needed(*contract_old_index);
//                         let contract_new_index = contract_remapping_queue.remapping(contract_old_index).unwrap();
//                         items.push(BlockItem {
//                             assumes: HashMap::new(),
//                             content: BlockItemContent::Contract(contract_new_index),
//                         })
//                     }
//                 }
//             },
//             RBlockLine::Function(_) => (),
//             RBlockLine::Bytes(_) => (),
//         }
//     }

//     Ok(Block { items })
// }

// #[derive(Default, Clone, Debug)]
// pub struct RemappingQueue<T> {
//     queue: Vec<T>,
//     remappings: HashMap<T, usize>,
// }

// impl<T: Eq + Hash + Clone> RemappingQueue<T> {
//     pub fn insert_if_needed(&mut self, item: T) -> bool {
//         if self.remapping(&item).is_some() {
//             false
//         } else {
//             self.remappings.insert(item.clone(), self.remappings.len());
//             self.queue.push(item);
//             true
//         }
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.queue.pop()
//     }

//     pub fn remapping(&self, item: &T) -> Option<usize> {
//         self.remappings.get(item).map(|x| *x)
//     }
// }
