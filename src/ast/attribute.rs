use std::ops::Deref;

use crate::parser::parser::FromPair;
use crate::parser::parser::{get_next, map_unique_child, Located, Rule};
use pest::iterators::Pair;

use super::literal::RHexOrStringLiteral;
use super::variable::RVariable;

#[derive(Debug, Clone)]
pub struct REquality {
    pub name: Located<RVariable>,
    pub value: Located<RHexOrStringLiteral>,
}

impl REquality {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for REquality {
    fn from_pair(equality: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(equality.as_rule() == Rule::equality);

        let mut inner = equality.into_inner();

        let name = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        _ = get_next(&mut inner, Rule::eq);

        let value = Located::<RHexOrStringLiteral>::from_pair(get_next(
            &mut inner,
            Rule::hex_or_string_literal,
        ))?;

        assert!(inner.next() == None);

        Ok(Self { name, value })
    }
}

#[derive(Debug, Clone)]
pub enum RAttributeArg {
    Variable(RVariable),
    Literal(RHexOrStringLiteral),
    Equality(REquality),
}

impl From<RVariable> for RAttributeArg {
    fn from(value: RVariable) -> Self {
        Self::Variable(value)
    }
}

impl From<RHexOrStringLiteral> for RAttributeArg {
    fn from(value: RHexOrStringLiteral) -> Self {
        Self::Literal(value)
    }
}

impl From<REquality> for RAttributeArg {
    fn from(value: REquality) -> Self {
        Self::Equality(value)
    }
}

impl FromPair for RAttributeArg {
    fn from_pair(attribute_arg: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute_arg.as_rule() == Rule::attribute_arg);

        map_unique_child(attribute_arg, |attribute_arg_inner| match attribute_arg_inner.as_rule() {
            Rule::equality => Ok(REquality::from_pair(attribute_arg_inner)?.into()),
            Rule::variable => Ok(RVariable::from_pair(attribute_arg_inner)?.into()),
            Rule::hex_or_string_literal => {
                Ok(RHexOrStringLiteral::from_pair(attribute_arg_inner)?.into())
            },
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RAttribute {
    pub name: Located<RVariable>,
    pub arg: Option<Located<RAttributeArg>>,
}

impl RAttribute {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for RAttribute {
    fn from_pair(attribute: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute.as_rule() == Rule::attribute);

        let mut attribute_inner = attribute.into_inner();

        let name = Located::<RVariable>::from_pair(get_next(&mut attribute_inner, Rule::variable))?;

        if let Some(paren) = attribute_inner.next() {
            assert!(paren.as_rule() == Rule::open_paren);

            let arg = Some(Located::<RAttributeArg>::from_pair(get_next(
                &mut attribute_inner,
                Rule::attribute_arg,
            ))?);

            _ = get_next(&mut attribute_inner, Rule::close_paren);
            assert!(attribute_inner.next() == None);

            Ok(Self { name, arg })
        } else {
            Ok(Self { name, arg: None })
        }
    }
}

#[derive(Debug, Clone)]
pub struct WithAttributes<T> {
    pub attributes: Vec<Located<RAttribute>>,
    pub inner: T,
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
                    let attr_inner = T::from_pair(attr_or_item)?;
                    assert!(inner.next() == None);
                    return Ok(Self { attributes, inner: attr_inner });
                },
            }
        }
        unreachable!()
    }
}

impl<T> WithAttributes<T> {
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for WithAttributes<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
