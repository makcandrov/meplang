use pest::iterators::Pair;

use crate::ast::constant::RConstant;
use crate::parser::parser::{Rule, FromPair, get_next};
use crate::ast::attribute::{RAttribute, WithAttributes};
use crate::ast::block::RBlock;
use crate::parser::parser::Located;

#[derive(Debug, Clone)]
pub struct VarName(pub String);

impl FromPair for VarName {
    fn from_pair(var_name: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(var_name.as_rule() == Rule::var_name);

        Ok(VarName(var_name.as_str().to_owned()))
    }
}

impl VarName {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RContract {
    pub name: Located<VarName>,
    pub blocks: Vec<Located<WithAttributes<RBlock>>>,
    pub constants: Vec<Located<RConstant>>,
}

impl RContract {
    pub fn name(&self) -> &str {
        &self.name.name()
    }
}

impl FromPair for RContract {
    fn from_pair(contract_decl: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> where Self: Sized {
        assert!(contract_decl.as_rule() == Rule::contract_decl);
    
        let mut contract_decl_inner = contract_decl.into_inner();

        _ = get_next(&mut contract_decl_inner, Rule::contract_keyword);

        let name = Located::<VarName>::from_pair(
            get_next(&mut contract_decl_inner, Rule::var_name)
        )?;

        let mut blocks = Vec::<Located<WithAttributes<RBlock>>>::new();
        let mut constants = Vec::<Located<RConstant>>::new();
        while let Some(contract_item) = contract_decl_inner.next() {
            match contract_item.as_rule() {
                Rule::block_decl_with_attr => {
                    blocks.push(Located::<WithAttributes<RBlock>>::from_pair(contract_item)?);
                },
                Rule::const_decl => {
                    constants.push(Located::<RConstant>::from_pair(contract_item)?);
                },
                _ => unreachable!(),
            }
        }

        return Ok(Self {
            name,
            blocks,
            constants,
        })
    }
}
