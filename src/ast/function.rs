use pest::iterators::Pair;
use quick_impl::QuickImpl;

use super::variable::{RVariable, RVariableWithField};
use super::{RConcatenation, RHexAlias};
use crate::parser::parser::{get_next, map_unique_child, FromPair, Located, Rule};

#[derive(Debug, Clone, QuickImpl)]
pub enum RFunctionArg {
    #[quick_impl(impl From)]
    VariableWithField(RVariableWithField),
    #[quick_impl(impl From)]
    VariablesConcat(RConcatenation),
    #[quick_impl(impl From)]
    HexAlias(RHexAlias),
}

impl FromPair for RFunctionArg {
    fn from_pair(function_arg: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(function_arg.as_rule() == Rule::function_arg);

        map_unique_child(function_arg, |child| match child.as_rule() {
            Rule::variable_with_field => Ok(RVariableWithField::from_pair(child)?.into()),
            Rule::concatenation => Ok(RConcatenation::from_pair(child)?.into()),
            Rule::hex_alias => Ok(RHexAlias::from_pair(child)?.into()),
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

        let arg = Located::<RFunctionArg>::from_pair(get_next(&mut function_inner, Rule::function_arg))?;

        _ = get_next(&mut function_inner, Rule::close_paren);
        assert!(function_inner.next() == None);

        Ok(Self { name, arg })
    }
}
