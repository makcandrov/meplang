use pest::iterators::Pair;
use quick_impl::quick_impl_all;

use super::RHexLiteral;
use crate::parser::{
    error::PestError,
    parser::{FromPair, Located, Rule, get_next, map_unique_child},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RVariable(pub String);

impl FromPair for RVariable {
    fn from_pair(variable: Pair<Rule>) -> Result<Self, PestError> {
        assert!(variable.as_rule() == Rule::variable);

        Ok(RVariable(variable.as_str().to_owned()))
    }
}

impl RVariable {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RVariableWithField {
    pub variable: Located<RVariable>,
    pub field: Located<RVariable>,
}

impl FromPair for RVariableWithField {
    fn from_pair(variable_with_field: Pair<Rule>) -> Result<Self, PestError> {
        assert!(variable_with_field.as_rule() == Rule::variable_with_field);

        let mut inner = variable_with_field.into_inner();

        let variable = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        let _ = get_next(&mut inner, Rule::dot);

        let field = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        assert!(inner.next().is_none());

        Ok(Self { variable, field })
    }
}

#[derive(Debug, Clone)]
pub struct RCompileVariable(pub Located<RVariable>);

impl FromPair for RCompileVariable {
    fn from_pair(compile_variable: Pair<Rule>) -> Result<Self, PestError> {
        assert!(compile_variable.as_rule() == Rule::compile_variable);

        let mut compile_var_inner = compile_variable.into_inner();

        _ = get_next(&mut compile_var_inner, Rule::dol);

        let res =
            Located::<RVariable>::from_pair(get_next(&mut compile_var_inner, Rule::variable))?;

        _ = get_next(&mut compile_var_inner, Rule::dol);

        Ok(Self(res))
    }
}

impl RCompileVariable {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
#[quick_impl_all(impl From)]
pub enum RHexAlias {
    HexLiteral(RHexLiteral),
    Variable(RVariable),
    CompileVariable(RCompileVariable),
}

impl FromPair for RHexAlias {
    fn from_pair(hex_alias: Pair<Rule>) -> Result<Self, PestError> {
        assert!(hex_alias.as_rule() == Rule::hex_alias);

        map_unique_child(hex_alias, |child| match child.as_rule() {
            Rule::hex_literal => Ok(RHexLiteral::from_pair(child)?.into()),
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::compile_variable => Ok(RCompileVariable::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RConcatenation(pub Vec<Located<RHexAlias>>);

impl FromPair for RConcatenation {
    fn from_pair(variables_concat: Pair<Rule>) -> Result<Self, PestError> {
        assert!(variables_concat.as_rule() == Rule::concatenation);

        let mut variables_concat_inner = variables_concat.into_inner();

        let mut res = vec![Located::<RHexAlias>::from_pair(get_next(
            &mut variables_concat_inner,
            Rule::hex_alias,
        ))?];

        while let Some(at) = variables_concat_inner.next() {
            assert!(at.as_rule() == Rule::at);

            res.push(Located::<RHexAlias>::from_pair(get_next(
                &mut variables_concat_inner,
                Rule::hex_alias,
            ))?)
        }

        Ok(Self(res))
    }
}
