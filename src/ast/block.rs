use crate::parser::parser::FromPair;
use crate::parser::parser::{get_next, map_unique_child, Located, Rule};
use pest::iterators::Pair;

use super::attribute::WithAttributes;
use super::function::RFunction;
use super::literal::RHexLiteral;
use super::variable::{RVariable, RVariableOrVariableWithField};

#[derive(Debug, Clone)]
pub enum RBlockRef {
    Star(RVariable),
    Esp(RVariableOrVariableWithField),
}

impl FromPair for RBlockRef {
    fn from_pair(block_ref: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_ref.as_rule() == Rule::block_ref);

        let mut inner = block_ref.into_inner();

        let res = match inner.next().unwrap().as_rule() {
            Rule::star => Self::Star(RVariable::from_pair(inner.next().unwrap())?),
            Rule::esp => Self::Esp(RVariableOrVariableWithField::from_pair(inner.next().unwrap())?),
            _ => unreachable!(),
        };

        assert!(inner.next().is_none());

        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub enum RBlockItem {
    Variable(RVariable),
    Function(RFunction),
    HexLiteral(RHexLiteral),
    BlockRef(RBlockRef),
}

impl From<RVariable> for RBlockItem {
    fn from(value: RVariable) -> Self {
        Self::Variable(value)
    }
}

impl From<RFunction> for RBlockItem {
    fn from(value: RFunction) -> Self {
        Self::Function(value)
    }
}

impl From<RHexLiteral> for RBlockItem {
    fn from(value: RHexLiteral) -> Self {
        Self::HexLiteral(value)
    }
}

impl From<RBlockRef> for RBlockItem {
    fn from(value: RBlockRef) -> Self {
        Self::BlockRef(value)
    }
}

impl FromPair for RBlockItem {
    fn from_pair(block_item: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_item.as_rule() == Rule::block_item);

        map_unique_child(block_item, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::function => Ok(RFunction::from_pair(child)?.into()),
            Rule::hex_literal => Ok(RHexLiteral::from_pair(child)?.into()),
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

        let name =
            Located::<RVariable>::from_pair(get_next(&mut block_decl_inner, Rule::variable))?;

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
