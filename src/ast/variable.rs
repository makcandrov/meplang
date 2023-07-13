use pest::iterators::Pair;

use crate::parser::parser::get_next;
use crate::parser::parser::map_unique_child;
use crate::parser::parser::FromPair;
use crate::parser::parser::Located;
use crate::parser::parser::Rule;

use super::RHexLiteral;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RVariable(pub String);

impl FromPair for RVariable {
    fn from_pair(variable: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
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
    fn from_pair(variable_with_field: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(variable_with_field.as_rule() == Rule::variable_with_field);

        let mut inner = variable_with_field.into_inner();

        let variable = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        let _ = get_next(&mut inner, Rule::dot);

        let field = Located::<RVariable>::from_pair(get_next(&mut inner, Rule::variable))?;

        assert!(inner.next() == None);

        Ok(Self { variable, field })
    }
}

#[derive(Debug, Clone)]
pub enum RVariableOrVariableWithField {
    Variable(RVariable),
    VariableWithField(RVariableWithField),
}

impl From<RVariable> for RVariableOrVariableWithField {
    fn from(value: RVariable) -> Self {
        Self::Variable(value)
    }
}

impl From<RVariableWithField> for RVariableOrVariableWithField {
    fn from(value: RVariableWithField) -> Self {
        Self::VariableWithField(value)
    }
}

impl FromPair for RVariableOrVariableWithField {
    fn from_pair(
        variable_or_variable_with_field: Pair<Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        assert!(variable_or_variable_with_field.as_rule() == Rule::variable_or_variable_with_field);

        map_unique_child(variable_or_variable_with_field, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::variable_with_field => Ok(RVariableWithField::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum RVariableOrHexLiteral {
    Variable(RVariable),
    HexLiteral(RHexLiteral),
}

impl From<RVariable> for RVariableOrHexLiteral {
    fn from(variable: RVariable) -> Self {
        Self::Variable(variable)
    }
}

impl From<RHexLiteral> for RVariableOrHexLiteral {
    fn from(hex_literal: RHexLiteral) -> Self {
        Self::HexLiteral(hex_literal)
    }
}

impl FromPair for RVariableOrHexLiteral {
    fn from_pair(
        variable_or_hex_literal: Pair<Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        assert!(variable_or_hex_literal.as_rule() == Rule::variable_or_hex_literal);

        map_unique_child(variable_or_hex_literal, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::hex_literal => Ok(RHexLiteral::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RConcatenation(pub Vec<Located<RVariableOrHexLiteral>>);

impl FromPair for RConcatenation {
    fn from_pair(
        variables_concat: Pair<Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        assert!(variables_concat.as_rule() == Rule::concatenation);

        let mut variables_concat_inner = variables_concat.into_inner();


        let mut res = vec![
            Located::<RVariableOrHexLiteral>::from_pair(get_next(&mut variables_concat_inner, Rule::variable_or_hex_literal))?
        ];

        while let Some(at) = variables_concat_inner.next() {
            assert!(at.as_rule() == Rule::at);

            res.push(
                Located::<RVariableOrHexLiteral>::from_pair(get_next(&mut variables_concat_inner, Rule::variable_or_hex_literal))?
            )
        }

        Ok(Self(res))
    }
}
