use bytes::Bytes;
use pest::iterators::Pair;
use primitive_types::H256;
use crate::{parser::{parser::{Rule, FromPair}, error::new_error_from_pair}, ast::affectation::Litteral};

use super::affectation::Affectation;


#[derive(Default, Debug, Clone)]
pub struct Constant {
    pub name: String, 
    pub value: Bytes,
}

impl FromPair for Constant {
    fn from_pair(const_decl: Pair<Rule>) -> Result<Constant, pest::error::Error<Rule>> {
        assert!(const_decl.as_rule() == Rule::const_decl);

        let mut const_decl_inner = const_decl.into_inner();
        assert!(const_decl_inner.next().unwrap().as_rule() == Rule::const_keyword);
        let affectation = const_decl_inner.next().unwrap();
        assert!(affectation.as_rule() == Rule::affectation);
        
        assert!(const_decl_inner.next().unwrap().as_rule() == Rule::semicolon);
        assert!(const_decl_inner.next() == None);

        let Affectation { name, value } = Affectation::from_pair(affectation.clone())?;

        let value = match value {
            Litteral::String(_) => {
                return Err(new_error_from_pair(&affectation, "expected hex litteral".to_owned()))
            },
            Litteral::Bytes(value) => value,
        };
        Ok(Self { name, value })
    }
}

