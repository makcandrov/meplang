use bytes::Bytes;
use pest::iterators::Pair;
use crate::{parser::{parser::{Rule, FromPair, Located}, error::new_error_from_pair}, ast::affectation::RLitteral};

use super::{affectation::RAffectation, contract::VarName};


#[derive(Debug, Clone)]
pub struct RConstant {
    pub name: Located<VarName>, 
    pub value: Located<Bytes>,
}

impl FromPair for RConstant {
    fn from_pair(const_decl: Pair<Rule>) -> Result<RConstant, pest::error::Error<Rule>> {
        assert!(const_decl.as_rule() == Rule::const_decl);

        let mut const_decl_inner = const_decl.into_inner();
        assert!(const_decl_inner.next().unwrap().as_rule() == Rule::const_keyword);
        let affectation = const_decl_inner.next().unwrap();
        assert!(affectation.as_rule() == Rule::affectation);
        
        assert!(const_decl_inner.next().unwrap().as_rule() == Rule::semicolon);
        assert!(const_decl_inner.next() == None);

        let RAffectation { name, value } = RAffectation::from_pair(affectation.clone())?;

        let value = match value.inner {
            RLitteral::String(_) => {
                return Err(new_error_from_pair(&affectation, "expected hex litteral".to_owned()))
            },
            RLitteral::Bytes(value) => value,
        };
        Ok(Self { name, value })
    }
}

