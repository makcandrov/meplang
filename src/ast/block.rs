use pest::iterators::Pair;

use super::attribute::WithAttributes;
use super::function::RFunction;
use super::variable::{RVariable, RVariableWithField};
use super::RHexAlias;
use crate::parser::parser::{get_next, map_unique_child, FromPair, Located, Rule};

#[derive(Debug, Clone)]
pub enum RBlockRefStar {
    Variable(RVariable),
}

impl From<RVariable> for RBlockRefStar {
    fn from(variable: RVariable) -> Self {
        Self::Variable(variable)
    }
}

impl FromPair for RBlockRefStar {
    fn from_pair(block_ref_star: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_ref_star.as_rule() == Rule::block_ref_star);

        map_unique_child(block_ref_star, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum RBlockRefEsp {
    Variable(RVariable),
    VariableWithField(RVariableWithField),
}

impl From<RVariable> for RBlockRefEsp {
    fn from(variable: RVariable) -> Self {
        Self::Variable(variable)
    }
}

impl From<RVariableWithField> for RBlockRefEsp {
    fn from(variable_with_field: RVariableWithField) -> Self {
        Self::VariableWithField(variable_with_field)
    }
}

impl FromPair for RBlockRefEsp {
    fn from_pair(block_ref_esp: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_ref_esp.as_rule() == Rule::block_ref_esp);

        map_unique_child(block_ref_esp, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::variable_with_field => Ok(RVariableWithField::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum RBlockRef {
    Star(RBlockRefStar),
    Esp(RBlockRefEsp),
}

impl FromPair for RBlockRef {
    fn from_pair(block_ref: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_ref.as_rule() == Rule::block_ref);

        let mut inner = block_ref.into_inner();

        let res = match inner.next().unwrap().as_rule() {
            Rule::star => Self::Star(RBlockRefStar::from_pair(inner.next().unwrap())?),
            Rule::esp => Self::Esp(RBlockRefEsp::from_pair(inner.next().unwrap())?),
            _ => unreachable!(),
        };

        assert!(inner.next().is_none());

        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub enum RBlockItem {
    Function(RFunction),
    HexAlias(RHexAlias),
    BlockRef(RBlockRef),
}

impl From<RFunction> for RBlockItem {
    fn from(function: RFunction) -> Self {
        Self::Function(function)
    }
}

impl From<RHexAlias> for RBlockItem {
    fn from(hex_alias: RHexAlias) -> Self {
        Self::HexAlias(hex_alias)
    }
}

impl From<RBlockRef> for RBlockItem {
    fn from(block_ref: RBlockRef) -> Self {
        Self::BlockRef(block_ref)
    }
}

impl FromPair for RBlockItem {
    fn from_pair(block_item: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_item.as_rule() == Rule::block_item);

        map_unique_child(block_item, |child| match child.as_rule() {
            Rule::function => Ok(RFunction::from_pair(child)?.into()),
            Rule::hex_alias => Ok(RHexAlias::from_pair(child)?.into()),
            Rule::block_ref => Ok(RBlockRef::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RBlock {
    pub name: Located<RVariable>,
    pub abstr: bool,
    pub items: Vec<Located<WithAttributes<Located<RBlockItem>>>>,
}

impl RBlock {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for RBlock {
    fn from_pair(block_decl: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_decl.as_rule() == Rule::block_decl);

        let mut block_decl_inner = block_decl.into_inner();

        let abstr = match block_decl_inner.next().unwrap().as_rule() {
            Rule::block_keyword => false,
            Rule::abstract_keyword => {
                _ = get_next(&mut block_decl_inner, Rule::block_keyword);
                true
            },
            _ => unreachable!(),
        };

        let name = Located::<RVariable>::from_pair(get_next(&mut block_decl_inner, Rule::variable))?;

        _ = get_next(&mut block_decl_inner, Rule::open_brace);

        let mut items = Vec::<Located<WithAttributes<Located<RBlockItem>>>>::new();
        while let Some(block_item_with_attr) = block_decl_inner.next() {
            match block_item_with_attr.as_rule() {
                Rule::block_item_with_attr => {
                    items.push(Located::<WithAttributes<Located<RBlockItem>>>::from_pair(
                        block_item_with_attr,
                    )?);
                },
                Rule::close_brace => {
                    assert!(block_decl_inner.next() == None);
                    return Ok(RBlock { name, abstr, items });
                },
                _ => unreachable!(),
            }
        }
        unreachable!();
    }
}
