use crate::parser::parser::{get_next, FromPair, Located, Rule};
use bytes::Bytes;
use pest::iterators::Pair;

use super::contract::VarName;

#[derive(Debug, Clone)]
pub struct RConstant {
    pub name: Located<VarName>,
    pub value: Located<Bytes>,
}

impl FromPair for RConstant {
    fn from_pair(const_decl: Pair<Rule>) -> Result<RConstant, pest::error::Error<Rule>> {
        assert!(const_decl.as_rule() == Rule::const_decl);

        let mut const_decl_inner = const_decl.into_inner();

        let _ = get_next(&mut const_decl_inner, Rule::const_keyword);

        let name = Located::<VarName>::from_pair(get_next(&mut const_decl_inner, Rule::var_name))?;

        let _ = get_next(&mut const_decl_inner, Rule::equal);

        let value =
            Located::<Bytes>::from_pair(get_next(&mut const_decl_inner, Rule::hex_litteral))?;

        let _ = get_next(&mut const_decl_inner, Rule::semicolon);
        assert!(const_decl_inner.next() == None);

        Ok(Self { name, value })
    }
}
