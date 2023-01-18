use std::str::FromStr;

use pest::iterators::Pair;
use crate::parser::parser::{Rule, Located, get_next, map_unique_child};
use crate::{opcode::OpCode, parser::parser::FromPair};

use super::affectation::{RAffectation, RLitteral};
use super::contract::VarName;


#[derive(Debug, Clone)]
pub struct RAttribute {
    pub name: Located<VarName>,
    pub arg: Option<Located<RAttributeArg>>,
}

impl FromPair for RAttribute {
    fn from_pair(attribute: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute.as_rule() == Rule::attribute);

        let mut attribute_inner = attribute.into_inner();

        let name = Located::<VarName>::from_pair(
            get_next(&mut attribute_inner, Rule::var_name)
        )?;

        let arg = if let Some(arg) = attribute_inner.next() {
            let res = Located::<RAttributeArg>::from_pair(arg)?;
            assert!(attribute_inner.next() == None);
            Some(res)
        } else {
            None
        };

        Ok(Self { name, arg })
    }
}

#[derive(Debug, Clone)]
pub enum RAttributeArg {
    Var(VarName),
    Litteral(RLitteral),
    Aff(RAffectation),
}

impl FromPair for RAttributeArg {
    fn from_pair(attribute_arg: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute_arg.as_rule() == Rule::attribute_arg);

        map_unique_child(attribute_arg, |attribute_arg_inner| {
            match attribute_arg_inner.as_rule() {
                Rule::affectation => Ok(Self::Aff(RAffectation::from_pair(attribute_arg_inner)?)),
                Rule::var_name => Ok(Self::Var(VarName::from_pair(attribute_arg_inner)?)),
                Rule::litteral => Ok(Self::Litteral(RLitteral::from_pair(attribute_arg_inner)?)),
                _ => unreachable!(),
            }
        })
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
                    attributes.push(Located::<RAttribute>::from_pair(attr_or_item)?);
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
