use std::ops::Deref;

use pest::iterators::Pair;
use quick_impl::QuickImpl;

use super::variable::RVariable;
use super::{RCompileVariable, RHexLiteral, RStringLiteral};
use crate::parser::parser::{get_next, map_unique_child, FromPair, Located, Rule};

#[derive(Debug, Clone, QuickImpl)]
pub enum RAttributeEqualityRight {
    #[quick_impl(impl From)]
    HexLiteral(RHexLiteral),
    #[quick_impl(impl From)]
    CompileVariable(RCompileVariable),
    #[quick_impl(impl From)]
    StringLiteral(#[allow(unused)] RStringLiteral),
}

impl FromPair for RAttributeEqualityRight {
    fn from_pair(attribute_equality_right: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute_equality_right.as_rule() == Rule::attribute_equality_right);

        map_unique_child(attribute_equality_right, |inner| match inner.as_rule() {
            Rule::hex_literal => Ok(RHexLiteral::from_pair(inner)?.into()),
            Rule::compile_variable => Ok(RCompileVariable::from_pair(inner)?.into()),
            Rule::string_literal => Ok(RStringLiteral::from_pair(inner)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RAttributeEquality {
    pub name: Located<RVariable>,
    pub value: Located<RAttributeEqualityRight>,
}

impl RAttributeEquality {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for RAttributeEquality {
    fn from_pair(attribute_equality: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute_equality.as_rule() == Rule::attribute_equality);

        let mut inner = attribute_equality.into_inner();

        let name = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        _ = get_next(&mut inner, Rule::eq);

        let value =
            Located::<RAttributeEqualityRight>::from_pair(get_next(&mut inner, Rule::attribute_equality_right))?;

        assert!(inner.next() == None);

        Ok(Self { name, value })
    }
}

#[derive(Debug, Clone, QuickImpl)]
pub enum RAttributeArg {
    #[quick_impl(impl From)]
    AttributeEquality(RAttributeEquality),
    #[quick_impl(impl From)]
    Variable(RVariable),
    #[quick_impl(impl From)]
    StringLiteral(#[allow(unused)] RStringLiteral),
}

impl FromPair for RAttributeArg {
    fn from_pair(attribute_arg: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(attribute_arg.as_rule() == Rule::attribute_arg);

        map_unique_child(attribute_arg, |inner| match inner.as_rule() {
            Rule::attribute_equality => Ok(RAttributeEquality::from_pair(inner)?.into()),
            Rule::variable => Ok(RVariable::from_pair(inner)?.into()),
            Rule::string_literal => Ok(RStringLiteral::from_pair(inner)?.into()),
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
                    return Ok(Self {
                        attributes,
                        inner: attr_inner,
                    });
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
