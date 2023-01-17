use bytes::Bytes;
use pest::iterators::Pair;
use crate::parser::parser::{Rule, FromPair};

#[derive(Debug, Clone)]
pub struct Affectation {
    pub name: String,
    pub value: Litteral,
}

#[derive(Debug, Clone)]
pub enum Litteral {
    String(String),
    Bytes(Bytes),
}

impl FromPair for Affectation {
    fn from_pair(expr: Pair<Rule>) -> Result<Affectation, pest::error::Error<Rule>> {
        assert!(expr.as_rule() == Rule::affectation);

        let mut affectation_inner = expr.into_inner();

        let name = affectation_inner.next().unwrap();
        assert!(name.as_rule() == Rule::var_name);

        assert!(affectation_inner.next().unwrap().as_rule() == Rule::equal);

        let value = affectation_inner.next().unwrap();
        assert!(value.as_rule() == Rule::litteral);

        assert!(affectation_inner.next() == None);

        Ok(Self {
            name: name.as_str().to_owned(),
            value: Litteral::from_pair(value)?,
        })
    }
}

impl FromPair for Litteral {
    fn from_pair(litteral: Pair<Rule>) -> Result<Litteral, pest::error::Error<Rule>> {
        assert!(litteral.as_rule() == Rule::litteral);

        let mut litteral_inner = litteral.into_inner();
        let string_or_hex_litteral = litteral_inner.next().unwrap();

        let res = match string_or_hex_litteral.as_rule() {
            Rule::string_litteral => Litteral::String(String::from_pair(string_or_hex_litteral)?),
            Rule::hex_litteral => Litteral::Bytes(Bytes::from_pair(string_or_hex_litteral)?),
            _ => unreachable!(),
        };

        assert!(litteral_inner.next() == None);

        Ok(res)
    }
}
