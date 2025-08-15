use std::collections::HashMap;

use bytes::Bytes;
use quick_impl::quick_impl_all;

use super::opcode::*;
use super::pre_processing::get_compile_variable_value;
use crate::ast::{RAttribute, RAttributeArg, RAttributeEqualityRight};
use crate::parser::error::{PestError, new_error_from_located};
use crate::parser::parser::Located;
use crate::types::bytes32::Bytes32;

#[rustfmt::skip]
const fn is_assumable_opcode(op: OpCode) -> bool {
    match op {
        ADDRESS | ORIGIN | CALLER | CALLVALUE | CALLDATASIZE | GASPRICE | RETURNDATASIZE |
        BLOCKHASH | COINBASE | TIMESTAMP | NUMBER | DIFFICULTY /* | RANDOM | PREVRANDAO */ | GASLIMIT | CHAINID | 
        BASEFEE | MSIZE => true,
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[quick_impl_all(pub const is)]
pub enum Attribute {
    Assume { op: u8, v: Bytes32 },
    ClearAssume { op: u8 },
    Keep,
    Main,
    Last,
    Optimization(bool),
}

impl Attribute {
    pub fn is_contract_attribute(&self) -> bool {
        !self.is_main() && !self.is_last() && !self.is_keep()
    }

    pub fn is_block_attribute(&self) -> bool {
        true
    }

    pub fn is_abstract_block_attribute(&self) -> bool {
        !self.is_main() && !self.is_last() && !self.is_keep()
    }

    pub fn is_block_item_attribute(&self) -> bool {
        matches!(
            self,
            Self::Assume { op: _, v: _ } | Self::ClearAssume { op: _ }
        )
    }

    pub fn from_r_attribute(
        input: &str,
        r_attribute: &Located<RAttribute>,
        compile_variables: &HashMap<String, Bytes>,
    ) -> Result<Self, PestError> {
        let name = r_attribute.name_str();
        match name {
            "assume" => {
                let Some(arg) = &r_attribute.arg else {
                    return Err(new_error_from_located(
                        input,
                        r_attribute,
                        "Argument required after `assume` attribute - ex: #[assume(msize = 0x20)]",
                    ));
                };

                let RAttributeArg::AttributeEquality(eq) = &arg.inner else {
                    return Err(new_error_from_located(
                        input,
                        r_attribute,
                        "Expected equality - ex: #[assume(msize = 0x20)]",
                    ));
                };

                let bytes = match &eq.value.inner {
                    RAttributeEqualityRight::HexLiteral(hex_literal) => hex_literal.0.clone(),
                    RAttributeEqualityRight::CompileVariable(compile_variable) => {
                        get_compile_variable_value(input, compile_variable, compile_variables)?
                            .clone()
                    }
                    _ => {
                        return Err(new_error_from_located(
                            input,
                            &eq.value,
                            "Expected: \n hex literal - ex: #[assume(msize = 0x20)] \n or compile variable - ex: #[assume(chainid = $CHAINID$)]",
                        ));
                    }
                };

                if bytes.len() > 32 {
                    return Err(new_error_from_located(
                        input,
                        &eq.value,
                        "Hexadecimal literal must be less than 32 bytes",
                    ));
                }

                let Some(op) = str_to_op(&eq.name_str().to_lowercase()) else {
                    return Err(new_error_from_located(
                        input,
                        &eq.name,
                        &format!("Unknown opcode `{}`", &eq.name_str()),
                    ));
                };

                if is_assumable_opcode(op) {
                    let Some(formatted) = Bytes32::from_bytes(&bytes, true) else {
                        return Err(new_error_from_located(
                            input,
                            &eq.name,
                            "Literal exceeds 32 bytes.",
                        ));
                    };

                    Ok(Self::Assume { op, v: formatted })
                } else {
                    Err(new_error_from_located(
                        input,
                        &eq.name,
                        "Cannot assume this opcode",
                    ))
                }
            }
            "clear_assume" => {
                let Some(arg) = &r_attribute.arg else {
                    return Err(new_error_from_located(
                        input,
                        r_attribute,
                        "Argument required after `clear_assume` attribute - ex: #[clear_assume(returndatasize)]",
                    ));
                };

                let RAttributeArg::Variable(var) = &arg.inner else {
                    return Err(new_error_from_located(
                        input,
                        r_attribute,
                        "Opcode name required after `clear_assume` attribute - ex: #[clear_assume(returndatasize)]",
                    ));
                };

                let Some(op) = str_to_op(&var.as_str().to_lowercase()) else {
                    return Err(new_error_from_located(
                        input,
                        arg,
                        &format!("Unknown opcode `{}`", var.as_str()),
                    ));
                };

                if is_assumable_opcode(op) {
                    Ok(Self::ClearAssume { op })
                } else {
                    Err(new_error_from_located(
                        input,
                        arg,
                        "Cannot assume this opcode",
                    ))
                }
            }
            "enable_optimization" => Ok(Self::Optimization(true)),
            "disable_optimization" => Ok(Self::Optimization(false)),
            "keep" => Ok(Self::Keep),
            "main" => Ok(Self::Main),
            "last" => Ok(Self::Last),
            _ => Err(new_error_from_located(
                input,
                &r_attribute.name,
                &format!("Unknown attribute `{}`", name),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attributes {
    pub assumes: HashMap<u8, Bytes32>,
    pub optimization: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            assumes: HashMap::new(),
            optimization: true,
        }
    }
}

impl Attributes {
    pub fn apply(&mut self, attribute: Attribute) {
        match attribute {
            Attribute::Assume { op, v } => {
                self.assumes.insert(op, v);
            }
            Attribute::ClearAssume { op } => {
                self.assumes.remove(&op);
            }
            Attribute::Optimization(enabled) => self.optimization = enabled,
            _ => (),
        }
    }

    pub fn apply_many(&mut self, attributes: Vec<Attribute>) {
        for attribute in attributes {
            self.apply(attribute);
        }
    }
}
