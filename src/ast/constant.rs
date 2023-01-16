use pest::iterators::Pair;
use primitive_types::H256;
use crate::parser::parser::{Rule, FromPair};

use super::expression::{Expression, self};

#[derive(Default, Debug, Clone)]
pub struct Constant {
    pub name: String, 
    pub value: H256,
}

impl From<Expression> for Constant {
    fn from(expr: Expression) -> Self {
        Self {
            name: expr.name,
            value: expr.value,
        }
    }
}

impl FromPair for Constant {
    fn from_pair(const_decl: Pair<Rule>) -> Result<Constant, pest::error::Error<Rule>> {
        assert!(const_decl.as_rule() == Rule::const_decl);

        let mut const_decl_inner = const_decl.into_inner();
        assert!(const_decl_inner.next().unwrap().as_rule() == Rule::const_keyword);
        let expr = const_decl_inner.next().unwrap();
        assert!(expr.as_rule() == Rule::expression);
        assert!(const_decl_inner.next() == None);

        Ok(Expression::from_pair(expr).into())
    }
}

