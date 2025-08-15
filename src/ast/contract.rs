use pest::iterators::Pair;

use super::variable::RVariable;
use crate::ast::attribute::WithAttributes;
use crate::ast::block::RBlock;
use crate::ast::constant::RConstant;
use crate::parser::error::PestError;
use crate::parser::parser::{FromPair, Located, Rule, get_next};

#[derive(Debug, Clone)]
pub struct RContract {
    pub name: Located<RVariable>,
    pub blocks: Vec<Located<WithAttributes<Located<RBlock>>>>,
    pub constants: Vec<Located<RConstant>>,
}

impl RContract {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for RContract {
    fn from_pair(contract_decl: Pair<Rule>) -> Result<Self, PestError>
    where
        Self: Sized,
    {
        assert!(contract_decl.as_rule() == Rule::contract_decl);

        let mut contract_decl_inner = contract_decl.into_inner();

        _ = get_next(&mut contract_decl_inner, Rule::contract_keyword);

        let name =
            Located::<RVariable>::from_pair(get_next(&mut contract_decl_inner, Rule::variable))?;

        _ = get_next(&mut contract_decl_inner, Rule::open_brace);

        let mut blocks = Vec::<Located<WithAttributes<Located<RBlock>>>>::new();
        let mut constants = Vec::<Located<RConstant>>::new();
        while let Some(contract_item) = contract_decl_inner.next() {
            match contract_item.as_rule() {
                Rule::block_decl_with_attr => {
                    blocks.push(Located::<WithAttributes<Located<RBlock>>>::from_pair(
                        contract_item,
                    )?);
                }
                Rule::const_decl => {
                    constants.push(Located::<RConstant>::from_pair(contract_item)?);
                }
                Rule::close_brace => {
                    assert!(contract_decl_inner.next().is_none());
                    return Ok(Self {
                        name,
                        blocks,
                        constants,
                    });
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}
