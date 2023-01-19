use crate::parser::error::new_error_from_pair;
use crate::parser::parser::map_unique_child;
use crate::parser::parser::FromPair;
use crate::parser::parser::Rule;
use bytes::Bytes;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RHexLitteral(pub Bytes);

impl From<Bytes> for RHexLitteral {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

impl FromPair for RHexLitteral {
    fn from_pair(hex_litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(hex_litteral.as_rule() == Rule::hex_litteral);

        if hex_litteral.as_str().len() % 2 != 0 {
            return Err(new_error_from_pair(
                &hex_litteral,
                "Hex litterals must be odd size".to_owned(),
            ));
        }

        match hex::decode(hex_litteral.as_str().strip_prefix("0x").unwrap()) {
            Ok(decoded) => Ok(Bytes::from(decoded).into()),
            Err(err) => Err(new_error_from_pair(&hex_litteral, err.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RStringLitteral(pub String);

impl From<String> for RStringLitteral {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromPair for RStringLitteral {
    fn from_pair(string_litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(string_litteral.as_rule() == Rule::string_litteral);

        Ok(map_unique_child(string_litteral, |string_inner| {
            assert!(string_inner.as_rule() == Rule::string_inner);
            string_inner.as_str().to_owned().into()
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RHexOrStringLitteral {
    RHexLitteral(RHexLitteral),
    RStringLitteral(RStringLitteral),
}

impl From<RHexLitteral> for RHexOrStringLitteral {
    fn from(value: RHexLitteral) -> Self {
        Self::RHexLitteral(value)
    }
}

impl From<RStringLitteral> for RHexOrStringLitteral {
    fn from(value: RStringLitteral) -> Self {
        Self::RStringLitteral(value)
    }
}

impl FromPair for RHexOrStringLitteral {
    fn from_pair(litteral: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(litteral.as_rule() == Rule::hex_or_string_litteral);

        map_unique_child(litteral, |litteral_inner| match litteral_inner.as_rule() {
            Rule::hex_litteral => Ok(RHexLitteral::from_pair(litteral_inner)?.into()),
            Rule::string_litteral => Ok(RStringLitteral::from_pair(litteral_inner)?.into()),
            _ => unreachable!(),
        })
    }
}
