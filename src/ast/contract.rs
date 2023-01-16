use std::collections::HashMap;

use primitive_types::H256;

use crate::ast::constant::Constant;
use crate::parser::parser::{Rule, FromPair};
use crate::ast::attribute::Attribute;
use crate::ast::block::Block;

#[derive(Default, Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub blocks: Vec<Block>,
    pub constants: Vec<Constant>,
}

impl FromPair for Contract {
    fn from_pair(contract_decl_with_attr: pest::iterators::Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> where Self: Sized {
        assert!(contract_decl_with_attr.as_rule() == Rule::contract_decl_with_attr);
    
        let mut res = Self::default();
        for attr_or_block in contract_decl_with_attr.into_inner() {
            match attr_or_block.as_rule() {
                Rule::attribute => {

                },
                Rule::contract_decl => {
                    let mut contract_decl_inner = attr_or_block.into_inner();

                    assert!(contract_decl_inner.next().unwrap().as_rule() == Rule::contract_keyword);

                    let contract_name = contract_decl_inner.next().unwrap();
                    assert!(contract_name.as_rule() == Rule::var_name);
                    
                    res.name = contract_name.as_str().to_owned();

                    while let Some(contract_item) = contract_decl_inner.next() {
                        match contract_item.as_rule() {
                            Rule::block_decl_with_attr => {
                                res.blocks.push(Block::from_pair(contract_item)?);
                            },
                            Rule::const_decl => {
                                res.constants.push(Constant::from_pair(contract_item)?);

                            },
                            _ => unreachable!(),
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
        Ok(res)
    }
}