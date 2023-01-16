use pest::iterators::Pair;
use pest_derive::Parser;
use crate::parser::error::new_error_from_pair;

#[derive(Parser)]
#[grammar = "meplang.pest"]
pub struct MeplangParser;

pub trait FromPair {
    fn from_pair(expr: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> where Self: Sized;
}

impl FromPair for bytes::Bytes {
    fn from_pair(hex_litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(hex_litteral.as_rule() == Rule::hex_litteral);

        let hex_str = format!(
            "{}{}",
            if hex_litteral.as_str().len() % 2 == 0 {
                ""
            } else {
                "0"
            },
            hex_litteral.as_str()
        );

        match hex::decode(hex_str) {
            Ok(decoded) => Ok(decoded.into()),
            Err(err) => Err(new_error_from_pair(&hex_litteral, err.to_string())),
        }
    }
}
