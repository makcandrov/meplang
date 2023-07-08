use crate::ast::*;
use crate::parser::error::new_error_from_located;
use crate::parser::parser::{Located, Location, Rule};

use bytes::{BufMut, Bytes, BytesMut};
use std::collections::{HashMap, HashSet};

use super::queue::DedupQueue;
use super::attribute::Attribute;
use super::opcode::str_to_op;

#[derive(Clone, Debug)]
pub struct BlockFlow {
    pub items: Vec<BlockFlowItem>,
    pub end_attributes: Vec<Attribute>,
    pub strong_deps: DedupQueue<usize>,
    pub weak_deps: DedupQueue<usize>,
}

#[derive(Clone, Debug)]
pub enum BlockFlowItem {
    Bytes(Bytes),
    Contract(usize),
    BlockEsp(BlockFlowBlockRef),
    BlockStar(BlockFlowBlockRef),
    Push(BlockFlowPush),
}

#[derive(Clone, Debug)]
pub struct BlockFlowBlockRef {
    pub index: usize,
    pub location: Location,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub struct BlockFlowPush {
    pub inner: BlockFlowPushInner,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug)]
pub enum BlockFlowPushInner {
    Constant(Bytes),
    BlockPc(usize),
    BlockSize(usize),
}

pub fn analyze_block_flow(
    input: &str,
    r_block_with_attr: &Located<WithAttributes<Located<RBlock>>>,
    constants: &HashMap<String, Bytes>,
    contract_names: &HashMap<String, usize>,
    block_names: &HashMap<String, usize>,
    contract_dependencies: &mut HashSet<usize>,
) -> Result<BlockFlow, pest::error::Error<Rule>> {
    // log::info!("Analyzing flow block {}", r_block_with_attr.inner().name_str());

    let r_block = r_block_with_attr.inner();

    let mut items = Vec::<BlockFlowItem>::new();
    let mut current_bytes: Option<BytesMut> = None;
    let mut current_attributes = Vec::<Attribute>::new();

    let mut strong_deps = DedupQueue::<usize>::new();
    let mut weak_deps = DedupQueue::<usize>::new();

    for r_item_with_attr in &r_block.items {
        for r_attribute in &r_item_with_attr.attributes {
            let attribute = Attribute::from_r_attribute(input, r_attribute)?;
            if attribute.is_block_item_attribute() {
                current_attributes.push(attribute);
            } else {
                return Err(new_error_from_located(input, r_attribute, "Invalid line attribute."));
            }
        }

        let r_item = r_item_with_attr.inner();

        match &r_item.inner {
            RBlockItem::HexLitteral(hex_litteral) => {
                append_or_create_bytes(&mut current_bytes, &hex_litteral.0);
                continue;
            },
            RBlockItem::Variable(variable) => {
                if let Some(op) = str_to_op(variable.as_str()) {
                    push_or_create_bytes(&mut current_bytes, op);
                } else {
                    return Err(new_error_from_located(
                        input,
                        &r_item,
                        &format!("Unknown opcode `{}`.", variable.as_str()),
                    ));
                }
                continue;
            },
            _ => (),
        }

        if let Some(c_bytes) = current_bytes.take() {
            items.push(BlockFlowItem::Bytes(c_bytes.into()));
        }

        match &r_item.inner {
            RBlockItem::HexLitteral(_) => unreachable!(),
            RBlockItem::Variable(_) => unreachable!(),
            RBlockItem::BlockRef(RBlockRef::Star(variable)) => {
                let block_name = variable.as_str();
                let Some(block_index) = block_names.get(variable.as_str()) else {
                    return Err(new_error_from_located(
                        input,
                        r_item,
                        &format!("Block `{}` not found in this contract.", block_name)
                    ));
                };

                strong_deps.insert_if_needed(*block_index);
                items.push(BlockFlowItem::BlockStar(BlockFlowBlockRef {
                    index: *block_index,
                    location: r_item.location.clone(),
                    attributes: current_attributes,
                }));
                current_attributes = Vec::new();
            },
            RBlockItem::BlockRef(RBlockRef::Esp(RVariableOrVariableWithField::Variable(
                variable,
            ))) => {
                let block_name = variable.as_str();
                let Some(block_index) = block_names.get(variable.as_str()) else {
                    return Err(new_error_from_located(
                        input,
                        r_item,
                        &format!("Block `{}` not found in this contract.", block_name)
                    ));
                };

                strong_deps.insert_if_needed(*block_index);
                items.push(BlockFlowItem::BlockEsp(BlockFlowBlockRef {
                    index: *block_index,
                    location: r_item.location.clone(),
                    attributes: current_attributes,
                }));
                current_attributes = Vec::new();
            },
            RBlockItem::BlockRef(RBlockRef::Esp(
                RVariableOrVariableWithField::VariableWithField(variable_with_field),
            )) => {
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

                items.push(BlockFlowItem::Contract(*contract_index));
                contract_dependencies.insert(*contract_index);
            },
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
                    RFunctionArg::HexLitteral(hex_litteral) => {
                        BlockFlowPushInner::Constant(format_bytes(&hex_litteral.0))
                    },
                    RFunctionArg::Variable(variable) => {
                        let Some(constant_value) = constants.get(variable.as_str()) else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                &format!("Invalid opcode `{}`.", variable.as_str()),
                            ));
                        };

                        BlockFlowPushInner::Constant(format_bytes(constant_value))
                    },
                    RFunctionArg::VariableWithField(variable_with_field) => {
                        let field_name = variable_with_field.field.as_str();
                        let variable_name = variable_with_field.variable.as_str();
                        match field_name {
                            "pc" => {
                                if let Some(block_index) = block_names.get(variable_name) {
                                    weak_deps.insert_if_needed(*block_index);
                                    BlockFlowPushInner::BlockPc(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Block `{}` not found.", variable_name),
                                    ));
                                }
                            },
                            "size" => {
                                if let Some(block_index) = block_names.get(variable_name) {
                                    weak_deps.insert_if_needed(*block_index);
                                    BlockFlowPushInner::BlockSize(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Block `{}` not found.", variable_name),
                                    ));
                                }
                            },
                            _ => {
                                return Err(new_error_from_located(
                                    input,
                                    &variable_with_field.field,
                                    &format!("Unknown field `{}`.", field_name),
                                ))
                            },
                        }
                    },
                };

                items.push(BlockFlowItem::Push(BlockFlowPush {
                    inner: push,
                    attributes: current_attributes,
                }));
                current_attributes = Vec::new();
            },
        }
    }

    if let Some(c_bytes) = current_bytes.take() {
        items.push(BlockFlowItem::Bytes(c_bytes.into()));
    }

    Ok(BlockFlow { items, end_attributes: current_attributes, strong_deps, weak_deps })
}

pub fn append_or_create_bytes(current_bytes: &mut Option<BytesMut>, new_bytes: &Bytes) {
    if let Some(c_bytes) = current_bytes.as_mut() {
        c_bytes.extend_from_slice(new_bytes);
    } else {
        current_bytes.replace(new_bytes[..].into());
    }
}

pub fn push_or_create_bytes(current_bytes: &mut Option<BytesMut>, new_byte: u8) {
    if let Some(c_bytes) = current_bytes.as_mut() {
        c_bytes.put_u8(new_byte);
    } else {
        let mut c_bytes = BytesMut::new();
        c_bytes.put_u8(new_byte);
        current_bytes.replace(c_bytes);
    }
}

pub fn format_bytes(bytes: &Bytes) -> Bytes {
    let mut i = 0;
    while i < bytes.len() && bytes[i] == 0x00 {
        i += 1;
    }
    if i == bytes.len() {
        return Bytes::new();
    }

    bytes.slice(i..bytes.len())
}
