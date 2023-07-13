use crate::parser::error::new_error_from_pair;
use crate::parser::parser::map_unique_child;
use crate::parser::parser::FromPair;
use crate::parser::parser::Rule;
use bytes::Bytes;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RHexLiteral(pub Bytes);

impl From<Bytes> for RHexLiteral {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

impl FromPair for RHexLiteral {
    fn from_pair(hex_literal: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(hex_literal.as_rule() == Rule::hex_literal);

        if hex_literal.as_str().len() % 2 != 0 {
            return Err(new_error_from_pair(
                &hex_literal,
                "Hex literals must be odd size.".to_owned(),
            ));
        }

        match hex::decode(hex_literal.as_str().strip_prefix("0x").unwrap()) {
            Ok(decoded) => Ok(Bytes::from(decoded).into()),
            Err(err) => Err(new_error_from_pair(&hex_literal, err.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RStringLiteral(pub String);

impl From<String> for RStringLiteral {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl FromPair for RStringLiteral {
    fn from_pair(string_literal: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(string_literal.as_rule() == Rule::string_literal);

        Ok(map_unique_child(string_literal, |string_inner| {
            assert!(string_inner.as_rule() == Rule::string_inner);
            string_inner.as_str().to_owned().into()
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RHexOrStringLiteral {
    RHexLiteral(RHexLiteral),
    RStringLiteral(RStringLiteral),
}

impl From<RHexLiteral> for RHexOrStringLiteral {
    fn from(value: RHexLiteral) -> Self {
        Self::RHexLiteral(value)
    }
}

impl From<RStringLiteral> for RHexOrStringLiteral {
    fn from(value: RStringLiteral) -> Self {
        Self::RStringLiteral(value)
    }
}

impl FromPair for RHexOrStringLiteral {
    fn from_pair(literal: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(literal.as_rule() == Rule::hex_or_string_literal);

        map_unique_child(literal, |literal_inner| match literal_inner.as_rule() {
            Rule::hex_literal => Ok(RHexLiteral::from_pair(literal_inner)?.into()),
            Rule::string_literal => Ok(RStringLiteral::from_pair(literal_inner)?.into()),
            _ => unreachable!(),
        })
    }
}
