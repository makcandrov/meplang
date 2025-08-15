use std::collections::{HashMap, HashSet};

use bytes::{BufMut, Bytes, BytesMut};
use indexmap::IndexSet;

use super::attribute::Attribute;
use super::opcode::str_to_op;
use super::pre_processing::get_compile_variable_value;
use crate::ast::*;
use crate::parser::error::{PestError, new_error_from_located};
use crate::parser::parser::{Located, Location};
use crate::types::bytes32::Bytes32;

#[derive(Clone, Debug)]
pub struct BlockFlow {
    pub items: Vec<BlockFlowItem>,
    pub end_attributes: Vec<Attribute>,
    pub strong_deps: IndexSet<usize>,
    pub weak_deps: IndexSet<usize>,
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
    Constant(Bytes32),
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
    compile_variables: &HashMap<String, Bytes>,
) -> Result<BlockFlow, PestError> {
    // tracing::info!("Analyzing flow block {}", r_block_with_attr.inner().name_str());

    let r_block = r_block_with_attr.inner();

    let mut items = Vec::<BlockFlowItem>::new();
    let mut current_bytes: Option<BytesMut> = None;
    let mut current_attributes = Vec::<Attribute>::new();

    let mut strong_deps = IndexSet::<usize>::new();
    let mut weak_deps = IndexSet::<usize>::new();

    for r_item_with_attr in &r_block.items {
        for r_attribute in &r_item_with_attr.attributes {
            let attribute = Attribute::from_r_attribute(input, r_attribute, compile_variables)?;
            if attribute.is_block_item_attribute() {
                current_attributes.push(attribute);
            } else {
                return Err(new_error_from_located(
                    input,
                    r_attribute,
                    "Invalid line attribute.",
                ));
            }
        }

        let r_item = r_item_with_attr.inner();

        if let RBlockItem::HexAlias(hex_alias) = &r_item.inner {
            match hex_alias {
                RHexAlias::HexLiteral(hex_literal) => {
                    append_or_create_bytes(&mut current_bytes, &hex_literal.0);
                }
                RHexAlias::Variable(variable) => {
                    let variable_name = variable.as_str();
                    if let Some(op) = str_to_op(variable_name) {
                        push_or_create_bytes(&mut current_bytes, op);
                    } else if let Some(constant) = constants.get(variable_name) {
                        append_or_create_bytes(&mut current_bytes, constant);
                    } else {
                        return Err(new_error_from_located(
                            input,
                            r_item,
                            &format!("Unknown opcode or constant`{}`.", variable_name),
                        ));
                    }
                }
                RHexAlias::CompileVariable(compile_variable) => {
                    append_or_create_bytes(
                        &mut current_bytes,
                        get_compile_variable_value(input, compile_variable, compile_variables)?,
                    );
                }
            }
            continue;
        }

        if let Some(c_bytes) = current_bytes.take() {
            items.push(BlockFlowItem::Bytes(c_bytes.into()));
        }

        match &r_item.inner {
            RBlockItem::HexAlias(_) => unreachable!(),
            RBlockItem::BlockRef(RBlockRef::Star(RBlockRefStar::Variable(variable))) => {
                let block_name = variable.as_str();
                let Some(block_index) = block_names.get(block_name) else {
                    return Err(new_error_from_located(
                        input,
                        r_item,
                        &format!("Block `{}` not found in this contract.", block_name),
                    ));
                };

                strong_deps.insert(*block_index);
                items.push(BlockFlowItem::BlockStar(BlockFlowBlockRef {
                    index: *block_index,
                    location: r_item.location.clone(),
                    attributes: current_attributes,
                }));
                current_attributes = Vec::new();
            }
            RBlockItem::BlockRef(RBlockRef::Esp(block_ref_esp)) => match block_ref_esp {
                RBlockRefEsp::Variable(variable) => {
                    let block_name = variable.as_str();
                    let Some(block_index) = block_names.get(block_name) else {
                        return Err(new_error_from_located(
                            input,
                            r_item,
                            &format!("Block `{}` not found in this contract.", block_name),
                        ));
                    };

                    strong_deps.insert(*block_index);
                    items.push(BlockFlowItem::BlockEsp(BlockFlowBlockRef {
                        index: *block_index,
                        location: r_item.location.clone(),
                        attributes: current_attributes,
                    }));
                    current_attributes = Vec::new();
                }
                RBlockRefEsp::VariableWithField(variable_with_field) => {
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
                }
            },
            RBlockItem::Function(function) => {
                let function_name = function.name.as_str();

                let push_right = match function_name.to_lowercase().as_str() {
                    "push" | "rpush" => true,
                    "lpush" => false,
                    _ => {
                        return Err(new_error_from_located(
                            input,
                            &function.name,
                            &format!("Unknown function `{}`.", function_name),
                        ));
                    }
                };

                let push = match &function.arg.inner {
                    RFunctionArg::HexAlias(RHexAlias::HexLiteral(hex_literal)) => {
                        let Some(formatted) = Bytes32::from_bytes(&hex_literal.0, push_right)
                        else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                "Push content exceeds 32 bytes.",
                            ));
                        };

                        BlockFlowPushInner::Constant(formatted)
                    }
                    RFunctionArg::HexAlias(RHexAlias::Variable(variable)) => {
                        let Some(constant_value) = constants.get(variable.as_str()) else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                &format!("Unknown argument `{}`.", variable.as_str()),
                            ));
                        };

                        let Some(formatted) = Bytes32::from_bytes(constant_value, push_right)
                        else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                "Push content exceeds 32 bytes.",
                            ));
                        };

                        BlockFlowPushInner::Constant(formatted)
                    }
                    RFunctionArg::HexAlias(RHexAlias::CompileVariable(compile_variable)) => {
                        let bytes =
                            get_compile_variable_value(input, compile_variable, compile_variables)?;
                        let Some(formatted) = Bytes32::from_bytes(bytes, push_right) else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                "Push content exceeds 32 bytes.",
                            ));
                        };

                        BlockFlowPushInner::Constant(formatted)
                    }
                    RFunctionArg::VariableWithField(variable_with_field) => {
                        if !push_right {
                            return Err(new_error_from_located(
                                input,
                                &variable_with_field.variable,
                                "Left push can only take constants as argument.",
                            ));
                        }

                        let field_name = variable_with_field.field.as_str();
                        let variable_name = variable_with_field.variable.as_str();
                        match field_name {
                            "pc" => {
                                if let Some(block_index) = block_names.get(variable_name) {
                                    weak_deps.insert(*block_index);
                                    BlockFlowPushInner::BlockPc(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Block `{}` not found.", variable_name),
                                    ));
                                }
                            }
                            "size" => {
                                if let Some(block_index) = block_names.get(variable_name) {
                                    weak_deps.insert(*block_index);
                                    BlockFlowPushInner::BlockSize(*block_index)
                                } else {
                                    return Err(new_error_from_located(
                                        input,
                                        &variable_with_field.variable,
                                        &format!("Block `{}` not found.", variable_name),
                                    ));
                                }
                            }
                            _ => {
                                return Err(new_error_from_located(
                                    input,
                                    &variable_with_field.field,
                                    &format!("Unknown field `{}`.", field_name),
                                ));
                            }
                        }
                    }
                    RFunctionArg::VariablesConcat(concat) => {
                        let mut bytes = BytesMut::new();
                        for variable in &concat.0 {
                            let value = match &variable.inner {
                                RHexAlias::Variable(variable) => {
                                    let Some(constant_value) = constants.get(variable.as_str())
                                    else {
                                        return Err(new_error_from_located(
                                            input,
                                            &function.arg,
                                            &format!("Unknown argument `{}`.", variable.as_str()),
                                        ));
                                    };
                                    constant_value
                                }
                                RHexAlias::HexLiteral(hex_literal) => &hex_literal.0,
                                RHexAlias::CompileVariable(compile_variable) => {
                                    get_compile_variable_value(
                                        input,
                                        compile_variable,
                                        compile_variables,
                                    )?
                                }
                            };

                            bytes.extend_from_slice(value);
                        }

                        let Some(formatted) = Bytes32::from_bytes(&bytes.into(), push_right) else {
                            return Err(new_error_from_located(
                                input,
                                &function.arg,
                                "Push content exceeds 32 bytes.",
                            ));
                        };

                        BlockFlowPushInner::Constant(formatted)
                    }
                };

                items.push(BlockFlowItem::Push(BlockFlowPush {
                    inner: push,
                    attributes: current_attributes,
                }));
                current_attributes = Vec::new();
            }
        }
    }

    if let Some(c_bytes) = current_bytes.take() {
        items.push(BlockFlowItem::Bytes(c_bytes.into()));
    }

    Ok(BlockFlow {
        items,
        end_attributes: current_attributes,
        strong_deps,
        weak_deps,
    })
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

pub fn is_function_name(name: &str) -> bool {
    matches!(name.to_lowercase().as_str(), "push" | "lpush" | "rpush")
}
