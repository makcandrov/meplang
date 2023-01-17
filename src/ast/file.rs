use pest::Parser;
use pest::iterators::Pair;
use crate::parser::error::new_generic_error;
use crate::parser::parser::FromPair;
use crate::parser::parser::MeplangParser;
use crate::parser::parser::Rule;
use super::contract::Contract;
use crate::parser::parser::Ast;

#[derive(Default, Debug, Clone)]
pub struct MeplangFile(pub Vec<Ast<Contract>>);

impl MeplangFile {
    pub fn new(code: String) -> Result<Self, pest::error::Error<Rule>> {
        let mut pairs = MeplangParser::parse(Rule::file, &code)?;
        let Some(file) = pairs.next() else {
            return Err(new_generic_error("invalid file".to_owned()))
        };
        if pairs.next() != None {
            return Err(new_generic_error("invalid file".to_owned()));
        }

        MeplangFile::from_pair(file)
    }
}

impl FromPair for MeplangFile {
    fn from_pair(file: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(file.as_rule() == Rule::file);
    
        let mut contracts = Vec::<Ast<Contract>>::new();
        match file.as_rule() {
            Rule::file => {
                for contract_decl_with_attr in file.into_inner() {
                    match contract_decl_with_attr.as_rule() {
                        Rule::EOI => (),
                        Rule::contract_decl_with_attr => {
                            contracts.push(Ast::<Contract>::try_from(contract_decl_with_attr)?);
                        },
                        _ => unreachable!(),
                    }
                    
                }
            }
            _ => unreachable!(),
        }

        Ok(Self(contracts))
    }
}