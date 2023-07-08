use crate::ast::litteral::RHexLitteral;
use crate::parser::parser::{get_next, FromPair, Located, Rule};
use pest::iterators::Pair;

use super::variable::RVariable;

#[derive(Debug, Clone)]
pub struct RConstant {
    pub name: Located<RVariable>,
    pub value: Located<RHexLitteral>,
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

        let name =
            Located::<RVariable>::from_pair(get_next(&mut const_decl_inner, Rule::variable))?;

        let _ = get_next(&mut const_decl_inner, Rule::eq);

        let value = Located::<RHexLitteral>::from_pair(get_next(
            &mut const_decl_inner,
            Rule::hex_litteral,
        ))?;

        let _ = get_next(&mut const_decl_inner, Rule::semicolon);
        assert!(const_decl_inner.next() == None);

        Ok(Self { name, value })
    }
}
