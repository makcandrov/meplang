use crate::parser::parser::FromPair;
use crate::parser::parser::{get_next, map_unique_child, Located, Rule};
use pest::iterators::Pair;

use super::literal::RHexLiteral;
use super::variable::{RVariable, RVariableWithField};
use super::RConcatenation;

#[derive(Debug, Clone)]
pub enum RFunctionArg {
    Variable(RVariable),
    VariableWithField(RVariableWithField),
    VariablesConcat(RConcatenation),
    HexLiteral(RHexLiteral),
}

impl From<RVariable> for RFunctionArg {
    fn from(value: RVariable) -> Self {
        Self::Variable(value)
    }
}

impl From<RVariableWithField> for RFunctionArg {
    fn from(value: RVariableWithField) -> Self {
        Self::VariableWithField(value)
    }
}

impl From<RConcatenation> for RFunctionArg {
    fn from(value: RConcatenation) -> Self {
        Self::VariablesConcat(value)
    }
}

impl From<RHexLiteral> for RFunctionArg {
    fn from(value: RHexLiteral) -> Self {
        Self::HexLiteral(value)
    }
}

impl FromPair for RFunctionArg {
    fn from_pair(function_arg: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(function_arg.as_rule() == Rule::function_arg);

        map_unique_child(function_arg, |child| match child.as_rule() {
            Rule::variable => Ok(RVariable::from_pair(child)?.into()),
            Rule::variable_with_field => Ok(RVariableWithField::from_pair(child)?.into()),
            Rule::concatenation => Ok(RConcatenation::from_pair(child)?.into()),
            Rule::hex_literal => Ok(RHexLiteral::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RFunction {
    pub name: Located<RVariable>,
    pub arg: Located<RFunctionArg>,
}

impl FromPair for RFunction {
    fn from_pair(function: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(function.as_rule() == Rule::function);

        let mut function_inner = function.into_inner();

        let name = Located::<RVariable>::from_pair(get_next(&mut function_inner, Rule::variable))?;

        _ = get_next(&mut function_inner, Rule::open_paren);

        let arg =
            Located::<RFunctionArg>::from_pair(get_next(&mut function_inner, Rule::function_arg))?;

        _ = get_next(&mut function_inner, Rule::close_paren);
        assert!(function_inner.next() == None);

        Ok(Self { name, arg })
    }
}
