use std::str::FromStr;

use pest::iterators::Pair;
use crate::parser::parser::{Rule, Located};
use crate::{opcode::OpCode, parser::parser::FromPair};

use super::affectation::RAffectation;
use super::contract::VarName;


#[derive(Debug, Clone)]
pub struct RAttribute {
    pub name: Located<VarName>,
    pub arg: Option<Located<AttributeArg>>,
}

#[derive(Debug, Clone)]
pub enum AttributeArg {
    Var(VarName),
    Aff(RAffectation),
}

impl FromPair for RAttribute {
    fn from_pair(pair: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct WithAttributes<T> {
    pub attributes: Vec<Located<RAttribute>>,
    pub inner: Located<T>,
}

impl<T: FromPair> FromPair for WithAttributes<T> {
    fn from_pair(item_with_attr: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        let mut inner = item_with_attr.into_inner();

        let mut attributes = Vec::<Located<RAttribute>>::new();
        while let Some(attr_or_item) = inner.next() {
            match attr_or_item.as_rule() {
                Rule::attribute => {
                    
                },
                _ => {
                    return Ok(Self {
                        attributes,
                        inner: Located::<T>::from_pair(attr_or_item)?,
                    });
                },
            }
        }
        panic!()
    }
}

impl<T> WithAttributes<T> {
    pub fn inner_located(&self) -> &Located<T> {
        &self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner_located().inner
    }
}
