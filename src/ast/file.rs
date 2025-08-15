use pest::Parser;
use pest::iterators::Pair;

use super::contract::RContract;
use crate::ast::attribute::WithAttributes;
use crate::parser::error::{PestError, new_generic_error};
use crate::parser::parser::{FromPair, Located, MeplangParser, Rule};

#[derive(Default, Debug, Clone)]
pub struct RFile(pub Vec<Located<WithAttributes<Located<RContract>>>>);

impl RFile {
    pub fn new(code: String) -> Result<Self, PestError> {
        let mut pairs = MeplangParser::parse(Rule::file, &code)?;
        let Some(file) = pairs.next() else {
            return Err(new_generic_error("invalid file".to_owned()));
        };
        if pairs.next().is_some() {
            return Err(new_generic_error("invalid file".to_owned()));
        }

        RFile::from_pair(file)
    }
}

impl FromPair for RFile {
    fn from_pair(file: Pair<Rule>) -> Result<Self, PestError> {
        assert!(file.as_rule() == Rule::file);

        let mut contracts = Vec::<Located<WithAttributes<Located<RContract>>>>::new();
        match file.as_rule() {
            Rule::file => {
                for contract_decl_with_attr in file.into_inner() {
                    match contract_decl_with_attr.as_rule() {
                        Rule::EOI => (),
                        Rule::contract_decl_with_attr => {
                            contracts.push(
                                Located::<WithAttributes<Located<RContract>>>::from_pair(
                                    contract_decl_with_attr,
                                )?,
                            );
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(Self(contracts))
    }
}
