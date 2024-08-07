use pest::iterators::Pair;
use quick_impl::QuickImpl;

use super::variable::RVariable;
use super::RCompileVariable;
use crate::ast::literal::RHexLiteral;
use crate::parser::parser::{get_next, map_unique_child, FromPair, Located, Rule};

#[derive(Debug, Clone, QuickImpl)]
pub enum RConstantArg {
    #[quick_impl(impl From)]
    HexLiteral(RHexLiteral),
    #[quick_impl(impl From)]
    CompileVariable(RCompileVariable),
}

impl FromPair for RConstantArg {
    fn from_pair(constant_arg: Pair<Rule>) -> Result<RConstantArg, pest::error::Error<Rule>> {
        assert!(constant_arg.as_rule() == Rule::const_arg);

        map_unique_child(constant_arg, |child| match child.as_rule() {
            Rule::hex_literal => Ok(RHexLiteral::from_pair(child)?.into()),
            Rule::compile_variable => Ok(RCompileVariable::from_pair(child)?.into()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RConstant {
    pub name: Located<RVariable>,
    pub value: Located<RConstantArg>,
}

impl RConstant {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl FromPair for RConstant {
    fn from_pair(const_decl: Pair<Rule>) -> Result<RConstant, pest::error::Error<Rule>> {
        assert!(const_decl.as_rule() == Rule::const_decl);

        let mut const_decl_inner = const_decl.into_inner();

        let _ = get_next(&mut const_decl_inner, Rule::const_keyword);

        let name = Located::<RVariable>::from_pair(get_next(&mut const_decl_inner, Rule::variable))?;

        let _ = get_next(&mut const_decl_inner, Rule::eq);

        let value = Located::<RConstantArg>::from_pair(get_next(&mut const_decl_inner, Rule::const_arg))?;

        let _ = get_next(&mut const_decl_inner, Rule::semicolon);
        assert!(const_decl_inner.next() == None);

        Ok(Self { name, value })
    }
}
