use crate::{
    ast::{attribute::RAttributeArg, litteral::RHexOrStringLitteral},
    parser::{
        error::new_error_from_located,
        parser::{Located, Rule},
    },
};
use bytes::Bytes;
use std::collections::HashMap;

use crate::ast::attribute::RAttribute;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribute {
    Assume { op: u8, v: Bytes },
    Main,
    Last,
    Optimization(bool),
}

impl Attribute {
    pub fn is_contract_attribute(&self) -> bool {
        *self != Self::Main && *self != Self::Last
    }

    pub fn is_block_attribute(&self) -> bool {
        true
    }

    pub fn is_block_item_attribute(&self) -> bool {
        match self {
            Self::Assume { op: _, v: _ } => true,
            _ => false,
        }
    }

    pub fn from_r_attribute(
        input: &str,
        r_attribute: &Located<RAttribute>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let name = r_attribute.name_str();
        match name {
            "assume" => {
                let Some(arg) = &r_attribute.arg else {
                    return Err(new_error_from_located(
                        input,
                        &r_attribute,
                        "Argument required after `assume` attribute - ex: #[assume(msize = 0x20)]",
                    ))
                };

                let RAttributeArg::Equality(eq) = &arg.inner else {
                    return Err(new_error_from_located(
                        input,
                        &r_attribute,
                        "Expected equality - ex: #[assume(msize = 0x20)]",
                    ));
                };

                let RHexOrStringLitteral::RHexLitteral(hex_litteral) = &eq.value.inner else {
                    return Err(new_error_from_located(
                        input,
                        &eq.value,
                        "Expected hex litteral - ex: #[assume(msize = 0x20)]",
                    ));
                };

                let bytes = hex_litteral.0.clone();
                if bytes.len() > 32 {
                    return Err(new_error_from_located(
                        input,
                        &eq.value,
                        "Hexadecimal litteral must be less than 32 bytes",
                    ));
                }

                let op_name = eq.name_str().to_lowercase();
                Ok(Self::Assume {
                    op: match op_name.as_str() {
                        "address" => 0x20,
                        "balance" => 0x31,
                        "origin" => 0x32,
                        "caller" => 0x33,
                        "callvalue" => 0x34,
                        "calldatasize" => 0x36,
                        "gasprice" => 0x3a,
                        "returndatasize" => 0x3a,
                        "chainid" => 0x46,
                        "selfbalance" => 0x47,
                        "msize" => 0x59,
                        _ => {
                            return Err(new_error_from_located(
                                input,
                                &eq.name,
                                "Cannot assume this opcode",
                            ));
                        } 
                    },
                    v: bytes
                })
                
            }
            "enable_optimization" => Ok(Self::Optimization(true)),
            "disable_optimization" => Ok(Self::Optimization(false)),
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
    pub assumes: HashMap<u8, Bytes>,
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

impl Attributes {}
