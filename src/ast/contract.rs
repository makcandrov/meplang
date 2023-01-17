use pest::iterators::Pair;

use crate::ast::constant::Constant;
use crate::parser::parser::{Rule, FromPair, get_next};
use crate::ast::attribute::Attribute;
use crate::ast::block::Block;
use crate::parser::parser::Ast;

#[derive(Debug, Clone)]
pub struct VarName(pub String);

impl FromPair for VarName {
    fn from_pair(var_name: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(var_name.as_rule() == Rule::var_name);

        Ok(VarName(var_name.as_str().to_owned()))
    }
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: Ast<VarName>,
    pub attributes: Vec<Ast<Attribute>>,
    pub blocks: Vec<Ast<Block>>,
    pub constants: Vec<Ast<Constant>>,
}

impl FromPair for Contract {
    fn from_pair(contract_decl_with_attr: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> where Self: Sized {
        assert!(contract_decl_with_attr.as_rule() == Rule::contract_decl_with_attr);
    
        let mut inner = contract_decl_with_attr.into_inner();

        let mut attributes = Vec::<Ast<Attribute>>::new();
        while let Some(attr_or_contract) = inner.next() {
            match attr_or_contract.as_rule() {
                Rule::attribute => {
                    
                },
                Rule::contract_decl => {
                    let mut contract_decl_inner = attr_or_contract.into_inner();

                    _ = get_next(&mut contract_decl_inner, Rule::contract_keyword);

                    let name = Ast::<VarName>::try_from(
                        get_next(&mut contract_decl_inner, Rule::var_name)
                    )?;

                    let mut blocks = Vec::<Ast<Block>>::new();
                    let mut constants = Vec::<Ast<Constant>>::new();
                    while let Some(contract_item) = contract_decl_inner.next() {
                        match contract_item.as_rule() {
                            Rule::block_decl_with_attr => {
                                blocks.push(Ast::<Block>::try_from(contract_item)?);
                            },
                            Rule::const_decl => {
                                constants.push(Ast::<Constant>::try_from(contract_item)?);
                            },
                            _ => unreachable!(),
                        }
                    }

                    assert!(inner.next() == None);

                    return Ok(Self {
                        name,
                        attributes,
                        blocks,
                        constants,
                    })
                },
                _ => unreachable!(), 
            }
        }
        panic!()
    }
}
