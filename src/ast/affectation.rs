use bytes::Bytes;
use pest::iterators::Pair;
use crate::parser::parser::{Rule, FromPair, Located};

use super::contract::VarName;

#[derive(Debug, Clone)]
pub struct RAffectation {
    pub name: Located<VarName>,
    pub value: Located<RLitteral>,
}

#[derive(Debug, Clone)]
pub enum RLitteral {
    String(Located<String>),
    Bytes(Located<Bytes>),
}

impl FromPair for RAffectation {
    fn from_pair(expr: Pair<Rule>) -> Result<RAffectation, pest::error::Error<Rule>> {
        assert!(expr.as_rule() == Rule::affectation);

        let mut affectation_inner = expr.into_inner();

        let name = affectation_inner.next().unwrap();
        assert!(name.as_rule() == Rule::var_name);

        assert!(affectation_inner.next().unwrap().as_rule() == Rule::equal);

        let value = affectation_inner.next().unwrap();
        assert!(value.as_rule() == Rule::litteral);

        assert!(affectation_inner.next() == None);

        Ok(Self {
            name: Located::<VarName>::from_pair(name)?,
            value: Located::<RLitteral>::from_pair(value)?,
        })
    }
}

impl FromPair for RLitteral {
    fn from_pair(litteral: Pair<Rule>) -> Result<RLitteral, pest::error::Error<Rule>> {
        assert!(litteral.as_rule() == Rule::litteral);

        let mut litteral_inner = litteral.into_inner();
        let string_or_hex_litteral = litteral_inner.next().unwrap();

        let res = match string_or_hex_litteral.as_rule() {
            Rule::string_litteral => RLitteral::String(Located::<String>::from_pair(string_or_hex_litteral)?),
            Rule::hex_litteral => RLitteral::Bytes(Located::<Bytes>::from_pair(string_or_hex_litteral)?),
            _ => unreachable!(),
        };

        assert!(litteral_inner.next() == None);

        Ok(res)
    }
}
